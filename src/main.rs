use std::env;
use std::collections::HashMap;

use poise::serenity_prelude as serenity;
use priced_items::consolidate_prices;
use crate::commands::*;
use crate::pricing::*;
use crate::priced_items::Priced;
use crate::config::Config;

mod commands;
mod pricing;
mod config;

struct Data {
    config: Config,
    item_data: HashMap<String, Priced>
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
    dotenv::dotenv().expect("Failed to load .env file");

    let config = Config::load_env().expect("Failed to load config");
    let token = config.discord_token.clone();

    let item_data = match consolidate_prices().await {
        Ok(items) => items,
        Err(_) => {
            eprintln!("Failed to pull items");
            HashMap::new()
        }
    };

    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                help::help(),
                price::price()
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
                    item_data
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