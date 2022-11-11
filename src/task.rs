use std::sync::Arc;

use convert_case::{Case, Casing};
use poise::serenity_prelude::{Context, Webhook};

use crate::{data::Data, model::Hook};

pub async fn spawn_tasks(ctx: Context, data: Data) {
    let ctx = Arc::new(ctx);
    let data = Arc::new(data);
    info!("start tasks");
    tokio::spawn(async move {
        loop {
            info!("start task: post hooks");
            let ctx = ctx.clone();
            let data = data.clone();
            post_booru_image(ctx, data).await;
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    });
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

async fn post_booru_image(ctx: Arc<Context>, data: Arc<Data>) {
    let pool = data.pool.clone();
    let hooks = match Hook::list_all(&pool).await {
        Ok(hooks) => hooks,
        Err(e) => {
            error!("Failed to get hooks: {}", e);
            return;
        }
    };
    let mut futs = Vec::new();
    for hook in hooks {
        let ctx = ctx.clone();
        let data = data.clone();
        let fut = async move {
            let post = match data.client.get_random_post(&hook.tag).await {
                Ok(post) => post,
                Err(e) => {
                    error!("Failed to get post: {}", e);
                    return;
                }
            };
            let url = post.file_url;
            let webhook = match Webhook::from_id_with_token(
                ctx.http.clone(),
                hook.hook_id as u64,
                &hook.hook_token,
            )
            .await
            {
                Ok(webhook) => webhook,
                Err(e) => {
                    error!("Failed to get webhook: {}", e);
                    return;
                }
            };
            let fields = vec![
                Field {
                    name: "Artist".to_string(),
                    value: to_separate(post.tag_string_artist),
                    inline: true,
                },
                Field {
                    name: "Copyright".to_string(),
                    value: to_separate(post.tag_string_copyright),
                    inline: true,
                },
                Field {
                    name: "Character".to_string(),
                    value: to_separate(post.tag_string_character),
                    inline: false,
                },
            ];
            let description = post.tag_string_general.split_whitespace().collect::<Vec<_>>().join(", ");
            let res = webhook
                .execute(&ctx.http, false, |w| {
                    w.embeds(vec![serde_json::json!({
                        "title": "щ（゜ロ゜щ）",
                        "description": format!("```\n{}\n```", description),
                        "color": 0x71368A,
                        "image": {
                            "url": url
                        },
                        "fields": fields,
                        "timestamp": post.created_at.to_rfc3339(),
                    })])
                    .components(|c| {
                        c.create_action_row(|a| {
                            a.create_button(|b| {
                                b.label("View on Danbooru");
                                b.style(poise::serenity_prelude::ButtonStyle::Link);
                                b.url(format!("https://danbooru.donmai.us/posts/{}", post.id));
                                b
                            });
                            a
                        });
                        c
                    });
                    w
                })
                .await;
            if let Err(e) = res {
                error!("Failed to send webhook: {}", e);
            }
        };
        futs.push(fut);
    }
    futures::future::join_all(futs).await;
}

fn to_separate(s: String) -> String {
    if s.is_empty() {
        return "Unknown".to_string();
    }
    s.split_whitespace()
        .map(|s| {
            format!(
                "[{}](https://danbooru.donmai.us/posts?tags={})",
                s.replace('_', " ").to_case(Case::Title),
                s
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
}
