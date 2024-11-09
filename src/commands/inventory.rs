use std::collections::HashMap;

use serde::Deserialize;

use crate::{Context, Error};
use crate::Priced;
use crate::currency::exchange;

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


#[derive(Deserialize, Debug)]
pub struct SteamSummaryPlayer {
    personaname: String,
    avatarfull: String,
}

#[derive(Deserialize, Debug)]
pub struct SteamSummaryWrapper {
    pub players: Vec<SteamSummaryPlayer>,
}

#[derive(Deserialize, Debug)]
pub struct SteamSummaryResponse {
    pub response: SteamSummaryWrapper,
}

async fn compute_inventory_value(
    ctx: &Context<'_>,
    steamid64: i64,
) -> Result<(f64, i32), Box<dyn std::error::Error + Send + Sync>> {
    let item_data: &HashMap<String, Priced> = &ctx.data().item_data;
    let doppler_data: &HashMap<String, String> = &ctx.data().doppler_data;

    let steamweb: SteamWebResponse;
    {
        let steamweb_token = &ctx.data().config.steamweb_token;
        let steamweb_url = format!("https://www.steamwebapi.com/steam/api/inventory?steam_id={}&key={}&parse=0",
            steamid64,
            steamweb_token
        );
        let response = reqwest::get(steamweb_url).await?.text().await?;
        steamweb = serde_json::from_str(&response)?;
    }

    // I don't know why Valve formats like this:
    // 1. Place all descriptions into hash table by classid
    let classid_lookup: HashMap<String, SteamWebDescription> = steamweb.descriptions
        .into_iter()
        .map(|desc| (desc.classid.clone(), desc))
        .collect();

    // 2. For each asset, lookup corresponding classid and compute price
    let mut total_value = 0.0;
    // let mut total_count = 0;
    let mut total_success = 0;

    for asset in &steamweb.assets {
        // total_count += 1;
        if let Some(description) = classid_lookup.get(&asset.classid) {
            // this will need to be modified for dopplers
            let mut modified_hash_name = description.market_hash_name.clone();

            if let Some(doppler) = doppler_data.get(&description.icon_url) {
                modified_hash_name += &format!(" {}", doppler);
            }

            if let Some(price) = item_data.get(&modified_hash_name) {
                if let Some(value) = price.feather {
                    total_value += value;
                    total_success += 1;
                }
            }
        }
    }

    Ok((total_value, total_success))
}


/// Check CS2 inventory value
#[poise::command(
    slash_command, prefix_command,
    category = "Items",
)]
pub async fn inv(
    ctx: Context<'_>,
    #[description = "User to check CS2 inventory"]
    user: Option<serenity::User>
) -> Result<(), Error> {

    // Check for prefix_command
    if ctx.prefix() != "/" {
        let embed = serenity::CreateEmbed::default()
            .title("This command is deprecated")
            .description("Please use </inv:1254551801647337494> instead.")
            .color(serenity::Color::RED);

        let reply = poise::CreateReply::default().embed(embed);

        ctx.send(reply).await?;

        return Ok(());
    }

    const ICON_URL: &str = "https://cdn.discordapp.com/avatars/371822760499871756/1caf027942b849dd774030ec8b333c10.webp";
    let mut embed = serenity::CreateEmbed::default()
        .author(serenity::CreateEmbedAuthor::new("Feather Inventory Valuation").icon_url(ICON_URL)).to_owned();
    let mut components = None;

    let target = user.as_ref().unwrap_or_else(|| ctx.author());
    let user_id = target.id.get() as i64;
    let author_id = ctx.author().id.get() as i64;
    let is_self: bool = user_id == author_id;

    let db = ctx.data().db.lock().await;

    if let (Some(target_user), Some(author_user)) = (db.get_user(&user_id).await?, db.get_user(&author_id).await?) {
        // Need to pull author_user to do potential currency conversion

        if target_user.steam_id == 0 {
            embed = embed.color(serenity::Color::RED);

            if is_self {
                embed = embed.title(":x:  Your steam account is not linked");

                components = Some(vec![serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new_link("http://feather.theseven.dev/auth/discord")
                        .label("Link Account")
                ])]);
            }
            else {
                embed = embed.title(":x:  This user's steam account is not linked")
            }
        } else {
            // Steam account is linked, check if we can evaluate or not

            match compute_inventory_value(&ctx, target_user.steam_id).await {
                Ok((inv_value, item_count)) => {
                    let steam_summary: SteamSummaryResponse;
                    {
                        let steam_token = &ctx.data().config.steam_token;
                        let steam_summary_url = format!("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
                            steam_token,
                            target_user.steam_id
                        );
                        let response = reqwest::get(steam_summary_url).await?.text().await?;
                        steam_summary = serde_json::from_str(&response)?;
                    }

                    let steam_player: &SteamSummaryPlayer = &steam_summary.response.players[0];

                    embed = embed.title(format!("{}'s CS2 Inventory", steam_player.personaname))
                        .description(format!("Account of <@{}>", user_id))
                        .thumbnail(steam_player.avatarfull.clone())
                        .color(serenity::Color::from_rgb(254, 171, 26))
                        .field(
                            format!("CS2 Inventory Value ({})", author_user.currency),
                            format!("**{}** items worth **{}**\n Powered by [CS Backpack](https://www.csbackpack.net/)",
                                item_count,
                                exchange(inv_value, &author_user.currency, &ctx).await
                            ),
                        false);

                    let server_id = ctx.guild_id().map(|id| id.get()).unwrap_or(0);
                    let referral_code = if server_id == 727970463325749268 {
                        "hade"
                    } else {
                        "botchicken"
                    };

                    components = Some(vec![serenity::CreateActionRow::Buttons(vec![
                        serenity::CreateButton::new_link(format!("https://steamcommunity.com/profiles/{}/inventory/730/", target_user.steam_id))
                            .label("View Inventory"),
                        serenity::CreateButton::new_link(format!("https://skinport.com/r/{}", referral_code))
                            .label("Purchase Skins"),
                    ])]);

                    // Perform role assignment
                    if is_self {
                        if let Some(guild_id) = ctx.guild_id() {
                            let g_id = guild_id.get() as i64;
                            if let Some(current_guild) = db.get_guild(&g_id).await? {
                                for role in current_guild.roles {
                                    if inv_value >= role.threshold {
                                        if let Some((role_id, _)) = guild_id.roles(&ctx).await?.iter().find(|(x, _)| x.get() as i64 == role.role_id) {
                                            let member = guild_id.member(&ctx, ctx.author().id).await?;

                                            match member.add_role(&ctx, role_id).await {
                                                Ok(_) => {
                                                    embed = embed.field("Role Assigned", format!("<@&{}>", role_id.get()), false);
                                                },
                                                Err(e) => {
                                                    embed = embed.field("Error adding role", e.to_string(), false);
                                                }
                                            }
                                        }

                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                Err(_) => {
                    embed = embed
                        .title(":x:  Error: Something unexpected occurred")
                        .description("Please make sure your Steam inventory is public")
                        .color(serenity::Color::RED)
                }
            }

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
