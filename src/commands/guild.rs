use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Configure inventory roles
#[poise::command(
    slash_command,
    category = "Guild settings",
    required_permissions = "MANAGE_GUILD",
    subcommands("list", "add")
)]
pub async fn invroles(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Display inventory roles
#[poise::command(
    slash_command,
    category = "Guild settings",
    required_permissions = "MANAGE_GUILD",
)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    
    Ok(())
}

/// Add an inventory role
#[poise::command(
    slash_command,
    category = "Guild settings",
    required_permissions = "MANAGE_GUILD",
)]
pub async fn add(
    ctx: Context<'_>,
    role: serenity::Role

) -> Result<(), Error> {

    Ok(())
}