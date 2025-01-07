use crate::{phrazes::PHRASES, Context, Error};
use ::serenity::all::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage, ReactionType};
use poise::{serenity_prelude as serenity, CreateReply};
use shuller::prelude::*;
use shuller::{random_usize, R34};

use super::image_board::MAX_TAB_LEN;

pub async fn paginate(
    ctx: Context<'_>,
    posts: Vec<Post>,
    show_tags: bool,
    show_reload_button: bool,
    pt: &[&str],
    nt: &[&str],
) -> Result<(), crate::Error> {
    let mut posts = posts;
    // let id of post
    let ctx_id = ctx.id();
    // current checked tab
    let mut current_tab = 0;

    let reply = create_reply(
        ctx,
        ctx_id,
        current_tab,
        &posts,
        show_reload_button,
        show_tags,
        false,
    );
    ctx.send(reply).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(60 * 60))
        .await
    {
        if press.data.custom_id == format!("luuma-{}-send_to_chat", ctx_id) {
            ctx.channel_id()
                .send_message(
                    &ctx.http(),
                    CreateMessage::new().embed(post(ctx, true, current_tab, show_tags, &posts)),
                )
                .await?;
        } else if press.data.custom_id == format!("luuma-{}-refresh", ctx_id) {
            posts.clear();
            let new_posts =
                R34!(D; p = pt.to_vec(), n = nt.to_vec(), limit = MAX_TAB_LEN as u16)
                    .map_err(|x| Error::from(format!(r#"**Error: Posts** not found. Err{}"#, x)))?;
            if posts.is_empty() {
                return Err("**Error: Posts** not found".into());
            }
            posts.extend(new_posts.data());
            current_tab = 0;
        } else {
            current_tab = {
                let temp: Vec<&str> = press.data.custom_id.split("-").collect();
                // luumma[0]-ctx_id[1]-id[2]-post[3]
                temp[2].parse().unwrap_or(0)
            };
        }
        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(post(ctx, false, current_tab, show_tags, &posts))
                        .components(vec![row_button(
                            ctx_id,
                            current_tab,
                            &posts,
                            show_reload_button,
                        )]),
                ),
            )
            .await?;
    }

    Ok(())
}

fn post(
    ctx: Context<'_>,
    is_public: bool,
    current_tab: usize,
    show_tags: bool,
    posts: &[Post],
) -> CreateEmbed {
    let mut embed = serenity::CreateEmbed::default();
    embed = embed
        .image(&posts[current_tab].sample_url)
        .url(&posts[current_tab].file_url)
        .title(PHRASES[random_usize!(PHRASES.len())])
        .color(serenity::Colour::from_rgb(
            random_usize!(255) as u8,
            random_usize!(255) as u8,
            random_usize!(255) as u8,
        ))
        .author(serenity::CreateEmbedAuthor::new(&posts[current_tab].owner));
    if !show_tags {
        embed = embed.description(format!(
            "
            [Post](https://rule34.xxx/index.php?page=post&s=view&id={})
            [Tag Link](https://rule34.xxx/index.php?page=post&s=list&tags={})
            ",
            &posts[current_tab].id,
            &posts[current_tab]
                .tags
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("+"),
        ));
    } else {
        embed = embed.description(format!(
            "
            [Post](https://rule34.xxx/index.php?page=post&s=view&id={})
            [Tag Link](https://rule34.xxx/index.php?page=post&s=list&tags={})
            **Tags:**
            ```{}```
            ",
            &posts[current_tab].id,
            &posts[current_tab]
                .tags
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("+"),
            &posts[current_tab].tags,
        ));
    }
    if is_public {
        embed = embed.footer(
            serenity::CreateEmbedFooter::new(format!("shared by: {}", ctx.author().name))
                .icon_url(ctx.author().avatar_url().unwrap_or_default()),
        );
    }
    embed
}

/// Init vec of buttons
fn buttons(
    ctx_id: u64,
    current_tab: usize,
    posts: &[Post],
    show_reload_button: bool,
) -> Vec<CreateButton> {
    // init row
    let mut vec = vec![];

    // gen enumerated buttons
    posts.iter().enumerate().for_each(|(num, _)| {
        let button = if current_tab == num {
            // current selected
            serenity::CreateButton::new(format!("luuma-{}-{}-post", ctx_id, num))
                .label(format!("{}", num + 1))
                .style(serenity::ButtonStyle::Success)
        } else {
            // just button
            serenity::CreateButton::new(format!("luuma-{}-{}-post", ctx_id, num))
                .label(format!("{}", num + 1))
                .style(serenity::ButtonStyle::Secondary)
        };
        vec.push(button)
    });
    // gen push button
    vec.push(
        serenity::CreateButton::new(format!("luuma-{}-send_to_chat", ctx_id))
            .emoji(ReactionType::Unicode("🚀".to_string()))
            .style(serenity::ButtonStyle::Primary),
    );
    // gen reload button
    if show_reload_button {
        vec.push(
            serenity::CreateButton::new(format!("luuma-{}-refresh", ctx_id))
                .emoji(ReactionType::Unicode("🔄".to_string()))
                .style(serenity::ButtonStyle::Secondary),
        );
    }
    vec
}

/// Buttons into Row
fn row_button(
    ctx_id: u64,
    current_tab: usize,
    posts: &[Post],
    show_reload_button: bool,
) -> CreateActionRow {
    let post = buttons(ctx_id, current_tab, posts, show_reload_button);
    serenity::CreateActionRow::Buttons(post)
}

fn create_reply(
    ctx: Context<'_>,
    ctx_id: u64,
    current_tab: usize,
    posts: &[Post],
    show_reload_button: bool,
    show_tags: bool,
    is_public: bool,
) -> CreateReply {
    let row_button = row_button(ctx_id, current_tab, posts, show_reload_button);
    CreateReply::default()
        .embed(post(ctx, is_public, current_tab, show_tags, posts))
        .components(vec![row_button])
}
