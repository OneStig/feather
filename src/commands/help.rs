
use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn help( ctx: Context<'_>) -> Result<(), Error> {
    let response = "Help";
    ctx.say(response).await?;
    Ok(())
}