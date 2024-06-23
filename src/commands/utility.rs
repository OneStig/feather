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
    let ISO_currency = currency.to_ascii_uppercase();
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

    Ok(())
}