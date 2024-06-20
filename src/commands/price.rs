use serenity::futures::{Stream, StreamExt};
use poise::serenity_prelude as serenity;
use crate::{Context, Error};

fn smart_search(item: &str, query: &str) -> bool {
    let item_lower = item.to_lowercase();
    let query_lower = query.to_lowercase();
    
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();
    
    query_words.iter().all(|&word| {
        item_lower.split(|c: char| !c.is_alphanumeric())
            .any(|item_part| item_part.contains(word))
    })
}
async fn autocomplete_item<'a>(
    ctx: Context<'a>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let item_data = &ctx.data().item_data;
    
    serenity::futures::stream::iter(item_data.keys())
        .filter(move |item| serenity::futures::future::ready(smart_search(item, partial)))
        .map(|item| item.to_string())
        .take(25)
}

#[poise::command(
    slash_command,
    category = "Items",
)]
pub async fn price(
    ctx: Context<'_>,
    #[description = "Item name"] #[rest]
    #[autocomplete = "autocomplete_item"]
    item_name: String
) -> Result<(), Error> {
    let footer = serenity::CreateEmbedFooter::new("Test footer");
    
    let reply = {
        let embed = serenity::CreateEmbed::default()
            .title(format!("Item: {}", item_name))
            .description("Feather is a CS2 item/inventory price checker")
            .color(serenity::Color::from((42, 55, 126)))
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