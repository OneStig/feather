use std::env;
use std::collections::HashMap;

use items::scrape_items;
use poise::serenity_prelude as serenity;
use crate::commands::*;
use crate::pricing::*;
use crate::config::Config;

mod commands;
mod pricing;
mod config;

struct Data {
    config: Config
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

    match scrape_items().await {
        Ok(items) => {
            for (key, item) in &items {
                // need to do some processing here
                // println!("{}", item.name);
            }
            println!("Total item count: {}", &items.len());
        }

        Err(_) => {
            eprintln!("Could not pull items");
        }
    }

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
                Ok(Data { config })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await;
    
    client.unwrap().start_autosharded().await.unwrap();
}