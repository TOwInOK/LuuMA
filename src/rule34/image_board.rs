use crate::{rule34::paginate, Context, Error};
use shuller::prelude::*;

static MAX_TAB_LEN: usize = 4;
static MAX_SEARCH_LEN: u16 = 30;

/// R34
/// - Video => animated & sound, to positive tags
/// - Gif => gif, to positive tags
/// - Img => any stuff
#[poise::command(
    slash_command,
    guild_only,
    nsfw_only,
    description_localized("ru", "Получить дозу эмоций"),
    name_localized("ru", "порно"),
    category = "NSFW",
    ephemeral
)]
pub async fn porno(
    ctx: Context<'_>,
    #[description = "let me your favorite tags!"]
    #[description_localized("ru", "Дай мне свои любимые теги")]
    #[name_localized("ru", "позитивные_теги")]
    positive_tags: Option<String>,
    #[description = "let me your unfavorite tags!"]
    #[description_localized("ru", "Что не нравится?")]
    #[name_localized("ru", "негативные_теги")]
    negative_tags: Option<String>,
    #[description = "Do you wanna some special?!"]
    #[description_localized("ru", "Позвони и узнай, как там с деньгами!")]
    #[name_localized("ru", "пробив_по_номеру")]
    id: Option<usize>,
    #[description = "How many do you want?"]
    #[description_localized("ru", "Сколько выдать?")]
    #[name_localized("ru", "количество")]
    size: Option<usize>,
    #[description = "Need to show tags?"]
    #[description_localized("ru", "Нужно ли отображать теги?")]
    #[name_localized("ru", "показать_теги")]
    show_tags: bool,
) -> Result<(), Error> {
    // Typing in chanel
    ctx.defer_ephemeral().await?;

    // init start bath of pictures
    let size = size.unwrap_or(4);

    // check max buttons (max 4)
    if size > MAX_TAB_LEN {
        return Err("**Error: TOOO BIG**, max size is 4".into());
    }

    // return in find
    if let Some(id) = id {
        // search id
        let posts = match R34!(D; id = id) {
            Ok(e) => e.data(),
            Err(_) => return Err("**Error: Posts** not found".into()),
        };
        // if empty
        if posts.is_empty() {
            return Err("**Error: Posts** not found".into());
        }
        // gen tab
        return paginate::paginate(ctx, &posts).await;
    }

    // positive tags
    let mut pt: Vec<&str> = vec![];
    // negative tags
    let mut nt: Vec<&str> = vec![];

    // build query

    let positive_tags = positive_tags.unwrap_or_default().to_lowercase();
    let negative_tags = negative_tags.unwrap_or_default().to_lowercase();

    // push positive
    for item in positive_tags.split_whitespace() {
        pt.push(item)
    }

    // push negative
    for item in negative_tags.split_whitespace() {
        nt.push(item)
    }

    // fetch posts with tags
    let posts = R34!(D; p = pt, n = nt, limit = MAX_SEARCH_LEN)
        .map_err(|x| Error::from(format!(r#"**Error: Posts** not found. Err{}"#, x)))?;
    if posts.is_empty() {
        return Err("**Error: Posts** not found".into());
    }
    // shuffle
    let mut posts_shuffled = posts.shuffle().data();
    if posts_shuffled.len() > MAX_TAB_LEN {
        posts_shuffled.truncate(MAX_TAB_LEN)
    }

    paginate::paginate(ctx, &posts_shuffled).await
}
