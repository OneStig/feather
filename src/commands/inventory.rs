use std::collections::HashMap;
use std::env;

use serde::Deserialize;

use crate::{Context, Error};
use crate::Priced;

use poise::serenity_prelude as serenity;

#[derive(Deserialize, Debug)]
pub struct SteamWebAsset {
    pub classid: String,
}

#[derive(Deserialize, Debug)]
pub struct SteamWebDescription {
    pub classid: String,
    pub market_hash_name: String,
    pub icon_url: String,
}

#[derive(Deserialize, Debug)]
pub struct SteamWebResponse {
    pub descriptions: Vec<SteamWebDescription>,
    pub assets: Vec<SteamWebAsset>,
}

async fn compute_inventory_value(
    item_data: &HashMap<String, Priced>,
    steamid64: i64,
    steamweb_token: &String
) -> Result<(f64, i32), Box<dyn std::error::Error + Send + Sync>> {
    let steamweb_url = format!("https://www.steamwebapi.com/steam/api/inventory?steam_id={}&key={}&parse=0",
        steamid64,
        steamweb_token
    );

    println!("{}", steamweb_url.clone());

    let response = reqwest::get(steamweb_url.clone()).await?.text().await?;
    let steamweb: SteamWebResponse = serde_json::from_str(&response)?;

    // I don't know why Valve formats like this:
    // 1. Place all descriptions into hash table by classid
    let classid_lookup: HashMap<String, SteamWebDescription> = steamweb.descriptions
        .into_iter()
        .map(|desc| (desc.classid.clone(), desc))
        .collect();

    // 2. For each asset, lookup corresponding classid and compute price
    let mut total_value = 0.0;
    let mut total_count = 0;
    let mut total_success = 0;

    for asset in &steamweb.assets {
        total_count += 1;
        if let Some(description) = classid_lookup.get(&asset.classid) {
            // this will need to be modified for dopplers
            let modified_hash_name = description.market_hash_name.clone();
            if let Some(price) = item_data.get(&modified_hash_name) {
                if let Some(value) = price.feather {
                    total_value += value;
                    total_success += 1;
                }
            }
        }
    }

    println!("inventory {}/{}", total_success, total_count);
    Ok((total_value, total_success))
}


/// Check CS2 inventory value
#[poise::command(
    slash_command,
    category = "Items",
)]
pub async fn inv(
    ctx: Context<'_>,
    #[description = "User to check CS2 inventory"]
    user: Option<serenity::User>
) -> Result<(), Error> {
    let is_self: bool = user == None;

    let target = user.as_ref().unwrap_or_else(|| ctx.author());
    let user_id = target.id.get() as i64;
    let author_id = ctx.author().id.get() as i64;
    
    let db = ctx.data().db.lock().await;
    
    let mut embed = serenity::CreateEmbed::default().to_owned();
    let mut components = None;
    
    
    if let (Some(target_user), Some(author_user)) = (db.get_user(&user_id).await?, db.get_user(&author_id).await?) {
        // Need to pull author_user to do potential currency conversion

        if target_user.steam_id == 0 {
            embed = embed.color(serenity::Color::RED);

            if is_self {
                embed = embed.title(":x:  Your steam account is not linked");

                components = Some(vec![serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new_link("http://feather.theseven.dev")
                        .label("Link Account")
                ])]);
            }
            else {
                embed = embed.title(":x:  This user's steam account is not linked")
            }
        } else {
            let (inv_value, item_count) = compute_inventory_value(&ctx.data().item_data, target_user.steam_id, &ctx.data().config.steamweb_token).await?;

            embed = embed.title("asdf").color(serenity::Color::from_rgb(0, 255, 0)).field("CS2 Inventory Value",
                format!("**{}** items worth **${:.2}**\n Powered by [CSGOSkinPrice](https://www.csgoskinprice.com/)",
                item_count,
                inv_value),
            false);

            components = Some(vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new_link(format!("https://steamcommunity.com/profiles/{}/inventory/730/", target_user.steam_id))
                    .label("View Inventory")
            ])]);
        }
    }
    else {
        embed = embed
            .title(":x:  Error: Something unexpected occurred")
            .color(serenity::Color::RED)
    }
    
    let mut reply = poise::CreateReply::default().embed(embed);
    
    if let Some(some_cmp) = components {
        reply = reply.components(some_cmp);
    }

    ctx.send(reply).await?;
    
    Ok(())
}