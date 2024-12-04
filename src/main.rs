use std::{collections::HashMap, sync::Arc};

use currency::load_exchange_rates;
// External crates
use poise::serenity_prelude as serenity;
use tokio::sync::Mutex;

// Local module imports
mod commands;
mod config;
mod database;
mod pricing;

// Re-exports from local
use commands::*;
use config::Config;
use database::DatabaseManager;
use pricing::*;
use priced_items::{consolidate_prices, Priced};

struct Data {
    config: Config,
    item_data: HashMap<String, Priced>,
    doppler_data: HashMap<String, String>,
    all_hash_names: Vec<String>,
    all_currency_codes: Vec<String>,
    currency_formats: HashMap<String, String>,
    conversion_rates: HashMap<String, f64>,
    db: Arc<Mutex<DatabaseManager>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn ready(&self, ctx: serenity::Context, ready: serenity::Ready) {
        println!("{} connected to shard #{}", ready.user.name, ctx.shard_id);
    }
}

#[tokio::main]
async fn main() {
    // Load info from .env file
    dotenv::dotenv().expect("Failed to load .env file");
    let config = Config::load_env().expect("Failed to load config");
    let token = config.discord_token.clone();

    // Load item information, but don't crash if fail
    let (item_data, doppler_data) = consolidate_prices().await.unwrap_or_else(|e| {
        eprintln!("Failed to pull items: {}", e);
        (HashMap::new(), HashMap::new())
    });

    let mut all_hash_names: Vec<String> = item_data.keys().cloned().collect();
    all_hash_names.sort_by(|a, b| {
        let count_a = a.split_whitespace().count();
        let count_b = b.split_whitespace().count();

        let word_count_cmp = count_a.cmp(&count_b);
        if word_count_cmp == std::cmp::Ordering::Equal {
            a.cmp(b)
        } else {
            word_count_cmp
        }
    });

    // Currency information
    let (currency_data, currency_formats) = load_exchange_rates().await.expect("Failed to load currencies");

    let mut all_currency_codes: Vec<String> = currency_formats.keys().cloned().collect();
    all_currency_codes.sort();

    let conversion_rates = currency_data.conversion_rates;

    // Load database manager, crash if fail
    let db = DatabaseManager::new().await.expect("Database failed to connect");

    // Register Discord gateway intents
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                help::help(),
                price::price(),

                inventory::inv(),

                guild::invroles(),
                guild::list(),
                guild::add(),
                guild::remove(),

                utility::currency(),
                utility::unlink(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("-".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { 
                    config,
                    item_data,
                    doppler_data,
                    all_hash_names,
                    all_currency_codes,
                    currency_formats,
                    conversion_rates,
                    db,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await;
    
    client.unwrap().start_autosharded().await.unwrap();
}