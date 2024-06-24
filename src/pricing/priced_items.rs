use std::collections::HashMap;
use std::fs::{self, File};
use std::io::copy;

use flate2::read::GzDecoder;
use serde::Deserialize;

use crate::items::*;

type Doppler = HashMap<String, Option<f64>>;

// steam
#[derive(Deserialize, Debug)]
pub struct SItem {
    last_24h: Option<f64>,
    last_7d: Option<f64>,
    last_30d: Option<f64>,
    last_90d: Option<f64>
}

// skinport
#[derive(Deserialize, Debug)]
pub struct PItem {
    starting_at: Option<f64>
}

// cstrade
#[derive(Deserialize, Debug)]
pub struct CItem {
    price: Option<f64>,
    doppler: Option<Doppler>
}

// buff
#[derive(Deserialize, Debug)]
pub struct BPrice {
    price: Option<f64>,
    doppler: Option<Doppler>
}

#[derive(Deserialize, Debug)]
pub struct BItem {
    starting_at: Option<BPrice>
}


#[derive(Deserialize, Debug)]
pub struct TItem {
    steam: SItem,
    skinport: Option<PItem>,
    cstrade: Option<CItem>,
    buff163: BItem,

    lootfarm: Option<f64>,
    csgoempire: Option<f64>,
    swapgg: Option<f64>,
    skinwallet: Option<f64>,
}

#[derive(Debug)]
pub struct Priced {
    pub info: Item,
    
    pub feather: Option<f64>,
    pub steam: Option<f64>,
    pub skinport: Option<f64>,
    pub buff: Option<f64>
}

const LOCAL_DOPPLER: &str = "doppler.json";
async fn load_doppler() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(LOCAL_DOPPLER)?;
    let dopplers: HashMap<String, String> = serde_json::from_str(&data)?;
    Ok(dopplers)
}

const LOCAL_PRICES: &str = "prices.json";
const API_PRICES: &str = "https://prices.csgotrader.app/latest/prices_v6.json";

async fn load_prices() -> Result<HashMap<String, TItem>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(LOCAL_PRICES)?;
    let items: HashMap<String, TItem> = serde_json::from_str(&data)?;
    Ok(items)
}

pub async fn refresh_prices() -> Result<HashMap<String, TItem>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(API_PRICES).send().await?;

    if response.status().is_success() {
        let bytes = response.bytes().await?;
        let mut gz = GzDecoder::new(&bytes[..]);
        let mut file = File::create(LOCAL_PRICES)?;
    
        copy(&mut gz, &mut file)?;
    }

    load_prices().await
}

fn power_mean(prices: &[f64]) -> Option<f64> {
    const POWER: f64 = -3.0;

    if prices.is_empty() {
        return None;
    }

    let n = prices.len() as f64;
    let sum: f64 = prices.iter().map(|&p| p.powf(POWER)).sum();

    Some((sum / n).powf(1.0 / POWER))
}

pub async fn consolidate_prices() -> Result<(HashMap<String, Priced>, HashMap<String, String>), Box<dyn std::error::Error>> {
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

    let doppler_data = load_doppler().await.expect("Failed to load doppler data");
    println!("Loaded doppler data");

    let mut priced_items: HashMap<String, Priced> = HashMap::new();
    let mut success_count = 0;

    for (_key, item) in &item_info {
        if let Some(hash_name) = &item.market_hash_name {
            priced_items.insert(
                if let Some(doppler_phase) = item.phase.clone() {
                    hash_name.clone() + " " + &doppler_phase
                } else {
                    hash_name.clone()
                },

                if let Some(item_price) = item_prices.get(hash_name) {
                    success_count += 1;

                    if let Some(doppler_phase) = item.phase.clone() {
                        let trader_price = item_price.cstrade
                            .as_ref()
                            .and_then(|cstrade| cstrade.doppler.as_ref())
                            .and_then(|doppler| doppler.get(&doppler_phase))
                            .and_then(|doppler_option| *doppler_option);

                        let buff_price = item_price.buff163.starting_at
                            .as_ref()
                            .and_then(|starting_at| starting_at.doppler.as_ref())
                            .and_then(|doppler| doppler.get(&doppler_phase))
                            .and_then(|doppler_option| *doppler_option);

                        Priced {
                            info: item.clone(),
                            feather: {
                                let prices: Vec<f64> = [
                                    trader_price,
                                    buff_price
                                ]
                                .iter()
                                .filter_map(|&price| price)
                                .collect();

                                power_mean(&prices) 
                            },
                            steam: None,
                            skinport: None,
                            buff: buff_price,
                        }
                    } else {
                        let mut steam_price = item_price.steam.last_24h
                            .or(item_price.steam.last_7d)
                            .or(item_price.steam.last_30d)
                            .or(item_price.steam.last_90d);
                        
                        if steam_price == Some(0.0) {
                            steam_price = None
                        }

                        let trader_price = item_price.cstrade
                            .as_ref()
                            .and_then(|cstrade| cstrade.price);
                        let skinport_price = item_price.skinport
                            .as_ref()
                            .and_then(|skinport| skinport.starting_at);
                        let buff_price = item_price.buff163.starting_at
                            .as_ref()
                            .and_then(|starting_at| starting_at.price);

                        Priced {
                            info: item.clone(),
                            feather: {
                                let mut prices: Vec<f64> = [
                                    steam_price,
                                    trader_price,
                                    skinport_price,
                                    buff_price,
                                    item_price.lootfarm,
                                    item_price.csgoempire,
                                    item_price.swapgg,
                                    item_price.skinwallet,
                                ]
                                .iter()
                                .filter_map(|&price| price)
                                .collect();
                                
                                prices.retain(|&price| price != 0.0);

                                power_mean(&prices) 
                            },
                            steam: steam_price,
                            skinport: skinport_price,
                            buff: buff_price,
                        }
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

    println!("Processed {}/{} items", success_count, priced_items.len());

    return Ok((priced_items, doppler_data));
}