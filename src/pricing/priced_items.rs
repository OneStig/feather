use std::collections::HashMap;
use std::fs;

use serde::Deserialize;

use crate::items::*;

#[derive(Deserialize, Debug)]
pub struct SItem {
    last_24h: Option<f32>,
    last_7d: Option<f32>,
    last_30d: Option<f32>,
    last_90d: Option<f32>
}

#[derive(Deserialize, Debug)]
pub struct TItem {
    steam: SItem
}

#[derive(Debug)]
pub struct Priced {
    pub info: Item,
    
    feather: Option<f32>,
    steam: Option<f32>,
    skinport: Option<f32>,
    buff: Option<f32>
}

const LOCAL_PRICES: &str = "prices.json";
const API_PRICES: &str = "https://prices.csgotrader.app/latest/prices_v6.json";

async fn load_prices() -> Result<HashMap<String, TItem>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(LOCAL_PRICES)?;
    let items: HashMap<String, TItem> = serde_json::from_str(&data)?;
    Ok(items)
}

pub async fn refresh_prices() -> Result<HashMap<String, TItem>, Box<dyn std::error::Error>> {
    // this is downloading garbage data
    let response = reqwest::get(API_PRICES).await?.text().await?;
    let items: HashMap<String, TItem> = serde_json::from_str(&response)?;
    fs::write(LOCAL_PRICES, response)?;
    Ok(items)
}

pub async fn consolidate_prices() -> Result<HashMap<String, Priced>, Box<dyn std::error::Error>> {
    let item_info = match scrape_items().await {
        Ok(items) => items,
        Err(e) => {
            return Err(e);
        }
    };

    let item_prices: HashMap<String, TItem> = match load_prices().await {
        Ok(items) => {
            println!("Loaded local {}", LOCAL_PRICES);
            items
        },
        Err(_) => {
            println!("Could not find {}", LOCAL_PRICES);
            match refresh_prices().await {
                Ok(items) => {
                    println!("Wrote new {}", LOCAL_PRICES);
                    items
                },
                Err(e) => {
                    println!("Failed to fetch prices");
                    return Err(e);
                }
            }
        }
    };

    let mut priced_items: HashMap<String, Priced> = HashMap::new();

    for (_key, item) in &item_info {
        if let Some(hash_name) = &item.market_hash_name {
            priced_items.insert(
                if let Some(doppler_phase) = item.phase.clone() {
                    hash_name.clone() + " " + &doppler_phase
                } else {
                    hash_name.clone()
                },

                if let Some(item_price) = item_prices.get(hash_name) {
                    Priced {
                        info: item.clone(),
                        feather: Some(1.0),
                        steam: Some(1.0),
                        skinport: Some(1.0),
                        buff: Some(1.0),
                    }
                } else {
                    Priced {
                        info: item.clone(),
                        feather: None,
                        steam: None,
                        skinport: None,
                        buff: None,
                    }
                }

            );
        }

    }

    println!("Processed {} items", priced_items.len());

    return Ok(priced_items);
}