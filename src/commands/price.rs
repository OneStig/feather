use serenity::futures::{Stream, StreamExt};
use poise::serenity_prelude as serenity;
use urlencoding::encode;

use crate::{Context, Error};
use crate::currency::exchange;

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
    let hash_names = &ctx.data().all_hash_names;

    serenity::futures::stream::iter(hash_names)
        .filter(move |item| serenity::futures::future::ready(smart_search(item, partial)))
        .map(|item| item.to_string())
        .take(15)
}

/// Check the price of a CS2 item
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
    let author_id = ctx.author().id.get() as i64;
    let db = ctx.data().db.lock().await;

    let reply = if let Some(found_skin) = &ctx.data().item_data.get(&item_name) {
        let author_user = db.get_user(&author_id).await?.unwrap();

        let rarity_color = match &found_skin.info.rarity {
            Some(rarity) => {
                serenity::Color::from_rgb(
                    u8::from_str_radix(&rarity.color[1..3], 16).unwrap(),
                    u8::from_str_radix(&rarity.color[3..5], 16).unwrap(),
                    u8::from_str_radix(&rarity.color[5..7], 16).unwrap()
                )
            },
            None => serenity::Color::LIGHT_GREY,
        };

        let mut embed = serenity::CreateEmbed::default()
            .title(format!("{}", item_name))
            .color(rarity_color)
            .fields(vec![
                ("<:botchicken:740299794550882324>  ·  Suggested Price", match found_skin.feather {
                    Some(p) => exchange(p, &author_user.currency, &ctx).await,
                    None => "Error".to_string()
                }, true),
                ("<:steam:740300441044123669>  ·  Steam Market", match found_skin.steam {
                    Some(p) => exchange(p, &author_user.currency, &ctx).await,
                    None => "Error".to_string()
                }, true),
                ("<:skinport:747619241250783353>  ·  Skinport", match found_skin.skinport {
                    Some(p) => exchange(p, &author_user.currency, &ctx).await,
                    None => "Error".to_string()
                }, true),
                ("<:buff163:801522918776766526>  ·  buff.163", match found_skin.buff {
                    Some(p) => exchange(p, &author_user.currency, &ctx).await,
                    None => "Error".to_string()
                }, true),
            ])
            .to_owned();

        match &found_skin.info.image {
            Some(imgurl) => {
                embed = embed.thumbnail(imgurl);
            }
            None => {}
        }

        let server_id = ctx.guild_id().map(|id| id.get()).unwrap_or(0);
        let referral_code = if server_id == 727970463325749268 {
            "hade"
        } else {
            "botchicken"
        };

        let encoded_item_name = encode(&item_name);

        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new_link(
                format!("https://skinport.com/market?r={}&search={}", referral_code, encoded_item_name)
            )
            .label("Purchase Item")
        ])];

        poise::CreateReply::default()
            .embed(embed)
            .components(components)
    } else {
        let embed = serenity::CreateEmbed::default()
            .title(":x:  Item could not be found")
            .color(serenity::Color::RED)
            .to_owned();

        poise::CreateReply::default()
            .embed(embed)
    };

    ctx.send(reply).await?;
    Ok(())
}
