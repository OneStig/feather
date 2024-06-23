use poise::serenity_prelude as serenity;
use serenity::futures::{Stream, StreamExt};
use crate::{Context, Error};

async fn autocomplete_currency<'a>(
    ctx: Context<'a>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    serenity::futures::stream::iter(&ctx.data().all_currency_codes)
        .filter(move |name| serenity::futures::future::ready(name.starts_with(&partial.to_ascii_uppercase())))
        .map(|name| name.to_string())
        .take(10)
}

/// Set preferred currency
#[poise::command(
    slash_command,
    category = "Utility")
]
pub async fn currency(
    ctx: Context<'_>,
    #[description = "Currency code"] #[rest]
    #[autocomplete = "autocomplete_currency"]
    currency: String
) -> Result<(), Error> {
    let iso_currency = currency.to_ascii_uppercase();

    let user_id = ctx.author().id.get() as i64;
    let db = ctx.data().db.lock().await;

    const ICON_URL: &str = "https://cdn.discordapp.com/avatars/371822760499871756/1caf027942b849dd774030ec8b333c10.webp";
    let mut embed = serenity::CreateEmbed::default()
        .author(serenity::CreateEmbedAuthor::new("Feather Inventory Valuation").icon_url(ICON_URL)).to_owned();

    if let Some(mut target_user) = db.get_user(&user_id).await? {
        if ctx.data().all_currency_codes.contains(&iso_currency) {
            target_user.currency = iso_currency.clone();
            db.update_user(&target_user).await?;

            embed = embed
                .title(format!("Your currency is set to {}", iso_currency))
                .color(serenity::Color::from_rgb(0, 255, 0))
        }
        else {
            embed = embed
                .title("Invalid currency code")
                .color(serenity::Color::RED)
        }
    } else {
        embed = embed
            .title(":x:  Error: Something unexpected occurred")
            .color(serenity::Color::RED)
    }
    
    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;
    Ok(())
}

/// Unlink steam account
#[poise::command(
    slash_command,
    category = "Utility")
]
pub async fn unlink(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let user_id = ctx.author().id.get() as i64;
    let db = ctx.data().db.lock().await;

    const ICON_URL: &str = "https://cdn.discordapp.com/avatars/371822760499871756/1caf027942b849dd774030ec8b333c10.webp";
    let mut embed = serenity::CreateEmbed::default()
        .author(serenity::CreateEmbedAuthor::new("Feather Inventory Valuation").icon_url(ICON_URL)).to_owned();

    if let Some(mut target_user) = db.get_user(&user_id).await? {
        target_user.steam_id = 0;
        db.update_user(&target_user).await?;

        embed = embed
            .title("Your steam account is unlinked")
            .color(serenity::Color::from_rgb(0, 255, 0))
    } else {
        embed = embed
            .title(":x:  Error: Something unexpected occurred")
            .color(serenity::Color::RED)
    }
    
    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;
    Ok(())
}