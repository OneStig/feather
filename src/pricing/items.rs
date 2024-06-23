use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Clone, Debug)]
pub struct Rarity {
    pub color: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct Item {
    // pub id: String,
    pub market_hash_name: Option<String>,
    pub image: Option<String>,
    pub rarity: Option<Rarity>,

    pub phase: Option<String>
}

const LOCAL_FILE: &str = "all.json";
const API_URL: &str = "https://bymykel.github.io/CSGO-API/api/en/all.json";

async fn load_json() -> Result<HashMap<String, Item>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(LOCAL_FILE)?;
    let items: HashMap<String, Item> = serde_json::from_str(&data)?;
    Ok(items)
}

pub async fn refresh_json() -> Result<HashMap<String, Item>, Box<dyn std::error::Error>> {
    let response = reqwest::get(API_URL).await?.text().await?;
    let items: HashMap<String, Item> = serde_json::from_str(&response)?;
    fs::write(LOCAL_FILE, response)?;
    Ok(items)
}

pub async fn scrape_items() -> Result<HashMap<String, Item>, Box<dyn std::error::Error>> {
    let items: HashMap<String, Item> = match load_json().await {
        Ok(items) => {
            println!("Loaded local {}", LOCAL_FILE);
            items
        },
        Err(_) => {
            println!("Could not find {}", LOCAL_FILE);
            match refresh_json().await {
                Ok(items) => {
                    println!("Wrote new {}", LOCAL_FILE);
                    items
                },
                Err(e) => {
                    println!("Failed to fetch items");
                    return Err(e);
                }
            }
        }
    };

    return Ok(items);
}