use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command, category = "Utility")]
pub async fn help( ctx: Context<'_>) -> Result<(), Error> {
    let footer = serenity::CreateEmbedFooter::new("Test footer");
    
    let reply = {
        let embed = serenity::CreateEmbed::default()
            .title("Feather Help")
            .description("Feather is a CS2 item/inventory price checker")
            .color(serenity::Color::BLUE)
            .fields(vec![
                ("title", "body", true),
                ("title", "body", true),
            ])
            .footer(footer)
            .to_owned();

        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new_link(&ctx.data().config.invite_link)
                .label("Invite me!")
        ])];

        poise::CreateReply::default()
            .embed(embed)
            .components(components)
    };

    ctx.send(reply).await?;

    Ok(())
}