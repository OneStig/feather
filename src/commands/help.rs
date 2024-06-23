use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Help menu with list of commands
#[poise::command(
    slash_command,
    prefix_command,
    category = "Utility")
]
pub async fn help( ctx: Context<'_>) -> Result<(), Error> {
    let reply = {
        const ICON_URL: &str = "https://cdn.discordapp.com/avatars/371822760499871756/1caf027942b849dd774030ec8b333c10.webp";
        let embed = serenity::CreateEmbed::default()
            .author(serenity::CreateEmbedAuthor::new("Feather Help").icon_url(ICON_URL))
            .description("Feather is a CS2 item/inventory price checker")
            .color(serenity::Color::from((38, 59, 127)))
            .fields(vec![
                ("Pricecheck Items", "`/price`", true),
                ("Pricecheck Inventory", "`/inv`", true),
                ("Set currency", "`/currency`", true),
                ("Unlink steam", "`/currency`", true),
                ("Server settings", "`/invroles`", true),
                ("Support Server", "[Join Server](https://discord.gg/hh9v4eF)", true)
            ])
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