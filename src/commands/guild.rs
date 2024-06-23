use std::cmp::Ordering;

use crate::{Context, Error};
use crate::database::models::RoleAssignment;
use poise::serenity_prelude as serenity;

const NOT_GUILD_MSG: &str = "Command can only be used in a guild";

/// Configure inventory roles
#[poise::command(
    slash_command, guild_only,
    category = "Guild settings",
    required_permissions = "MANAGE_GUILD",
    subcommands("list", "add", "remove")
)]
pub async fn invroles(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Display inventory roles
#[poise::command(
    slash_command, guild_only,
    category = "Guild settings",
    required_permissions = "MANAGE_GUILD",
)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect(NOT_GUILD_MSG).get() as i64;
    let db = ctx.data().db.lock().await;

    let mut embed = serenity::CreateEmbed::default().to_owned();
    let mut components = None;

    if let Some(guild) = db.get_guild(&guild_id).await? {
        embed = embed
            .title("Server role settings")
            .color(serenity::Color::from_rgb(255, 255, 255));
        
        if guild.roles.is_empty() {
            embed = embed.field("No rules set for this guild", "Use </invroles add:1254143286298415156> to configure", false);
        }
        else {
            let mut list_string = String::new();

            for role_rule in guild.roles {
                list_string.push_str(&format!("<@&{}> at **${:.2}**\n", role_rule.role_id, role_rule.threshold));
            };
            embed = embed.field("Configured list", list_string, false);
        }
    } else {
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

/// Add an inventory role
#[poise::command(
    slash_command, guild_only,
    category = "Guild settings",
    required_permissions = "MANAGE_GUILD",
)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Role to assign"]
    role: serenity::Role,
    #[description = "USD amount to assign at"]
    threshold: f64
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect(NOT_GUILD_MSG).get() as i64;
    let db = ctx.data().db.lock().await;

    let mut embed = serenity::CreateEmbed::default().to_owned();
    let mut components = None;

    if let Some(mut guild) = db.get_guild(&guild_id).await? {
        guild.roles.push(
            RoleAssignment { threshold: threshold, role_id: role.id.get() as i64 }
        );

        guild.roles.sort_by(|a, b| b.threshold.partial_cmp(&a.threshold).unwrap_or(Ordering::Equal));

        db.update_guild(&guild).await?;

        embed = embed
            .title("Roles modified")
            .color(serenity::Color::from_rgb(255, 255, 255))
            .field("New role", format!("<@&{}> at **${:.2}**", role.id.get(), threshold), false);
    } else {
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

/// Role to remove
#[poise::command(
    slash_command, guild_only,
    category = "Guild settings",
    required_permissions = "MANAGE_GUILD",
)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Role to remove"]
    role: serenity::Role,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect(NOT_GUILD_MSG).get() as i64;
    let db = ctx.data().db.lock().await;

    let mut embed = serenity::CreateEmbed::default().to_owned();
    let mut components = None;

    if let Some(mut guild) = db.get_guild(&guild_id).await? {
        guild.roles.retain(|vec_role| vec_role.role_id != role.id.get() as i64);
        guild.roles.sort_by(|a, b| b.threshold.partial_cmp(&a.threshold).unwrap_or(Ordering::Equal));
        db.update_guild(&guild).await?;

        embed = embed
            .title("Roles modified")
            .color(serenity::Color::from_rgb(255, 255, 255))
            .field("Erased rules containing", format!("<@&{}>", role.id.get()), false);
    } else {
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