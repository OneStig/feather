use crate::{Context, Error};
use poise::serenity_prelude as serenity;

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
            // Query inventory api, compute value, then assign roles based on is_self

            embed = embed.title("asdf");

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