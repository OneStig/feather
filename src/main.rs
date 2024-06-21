use std::{collections::HashMap, sync::Arc};

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
use database::{DatabaseManager, User, Guild};
use pricing::*;
use priced_items::{consolidate_prices, Priced};

struct Data {
    config: Config,
    item_data: HashMap<String, Priced>,
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
    let item_data = match consolidate_prices().await {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Failed to pull items: {}", e);
            HashMap::new()
        }
    };
    
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
                inventory::inv()
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
                    db
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