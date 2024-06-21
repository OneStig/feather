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
    // println!("{}", target.id.get());
    
    
    Ok(())
}