use poise::serenity_prelude::{json::json, Color};

use crate::{client::Post, data::Data, error::BotError, model::Hook};

pub type Context<'a> = poise::Context<'a, Data, BotError>;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), BotError> {
    if let Err(e) = poise::builtins::register_application_commands_buttons(ctx).await {
        error!("Failed to register application commands: {}", e);
    }
    Ok(())
}

/// ランダムな画像を投稿します
#[poise::command(prefix_command, slash_command)]
pub async fn danbooru(
    ctx: Context<'_>,
    #[description = "tag"] tag: String,
) -> Result<(), BotError> {
    ctx.defer().await?;
    let data = ctx.data();
    let image = data.client.get_random_post(&tag).await?;
    debug!("post image: {:?}", image);
    send_image(&ctx, image).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, subcommands("new"))]
pub async fn autobooru(_ctx: Context<'_>) -> Result<(), BotError> {
    Ok(())
}

/// 自動投稿を開始します
#[poise::command(prefix_command, slash_command)]
pub async fn new(ctx: Context<'_>, #[description = "tag"] tag: String) -> Result<(), BotError> {
    ctx.defer().await?;
    let data = ctx.data();
    let guild_id = match ctx.guild_id() {
        Some(id) => id.0 as i64,
        None => {
            ctx.say("このコマンドはサーバー内でのみ使用できます")
                .await?;
            return Ok(());
        }
    };
    let channel_id = ctx.channel_id().0;
    let hook = Hook::get_by_guild_and_channel(&data.pool, guild_id, channel_id as i64).await?;
    if !hook.is_empty() {
        let s = "このチャンネルは既に登録済みです\n`/autobooru delete`で削除するか、`/autobooru edit`でタグを編集できます。\n現在のタグは";
        ctx.say(format!("{} `{}` です。", s, hook[0].tag)).await?;
        return Ok(());
    }
    let hook = ctx
        .discord()
        .http
        .create_webhook(channel_id, &json!({
            "name": "Autobooru".to_string(),
            "avavtar": "https://cdn.discordapp.com/app-icons/1031731551928590366/7fefc1692284d3a5bf332ce5ff476c0a.png?size=256".to_string()
        }), None)
        .await?;
    let hook_id = hook.id.0;
    let hook_token = match hook.token {
        Some(token) => token,
        None => {
            ctx.say("Webhookの作成に失敗しました").await?;
            return Ok(());
        }
    };
    Hook::insert(
        &data.pool,
        guild_id,
        channel_id as i64,
        hook_id as i64,
        hook_token,
        tag.clone(),
    )
    .await?;
    ctx.say(format!("自動投稿を開始しました\n\nタグ: `{}`", tag)).await?;
    Ok(())
}

async fn send_image(ctx: &Context<'_>, image: Post) -> Result<(), BotError> {
    ctx.send(|b| {
        b.embed(|e| {
            e.title("щ（゜ロ゜щ）");
            e.image(&image.file_url);
            e.color(Color::DARK_PURPLE);
            e
        })
        .components(|c| {
            c.create_action_row(|a| {
                a.create_button(|b| {
                    b.label("View on Danbooru");
                    b.style(poise::serenity_prelude::ButtonStyle::Link);
                    b.url(format!("https://danbooru.donmai.us/posts/{}", image.id));
                    b
                });
                a
            });
            c
        });
        b
    })
    .await?;
    Ok(())
}
