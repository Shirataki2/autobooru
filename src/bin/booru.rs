use autobooru::{commands, data::Data, error::on_error, handler::handle_event};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    env_logger::init();

    let discord_bot_token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN not set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let booru_username = std::env::var("BOORU_USERNAME").expect("BOORU_USERNAME not set");
    let booru_api_key = std::env::var("BOORU_API_KEY").expect("BOORU_API_KEY not set");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let client = autobooru::client::BooruClient::new(booru_username, booru_api_key);

    let opts = poise::FrameworkOptions {
        commands: vec![
            commands::register(),
            commands::danbooru(),
            commands::autobooru(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("ab.".to_owned()),
            edit_tracker: Some(poise::EditTracker::for_timespan(
                std::time::Duration::from_secs(3600),
            )),
            ignore_bots: true,
            ..Default::default()
        },
        listener: |ctx, ev, fw, data| Box::pin(handle_event(ctx, ev, fw, data)),
        on_error: |err| Box::pin(on_error(err)),
        ..Default::default()
    };

    poise::Framework::builder()
        .options(opts)
        .token(discord_bot_token)
        .user_data_setup(move |_ctx, _ready, _fw| {
            Box::pin(async move {
                Ok(Data {
                    pool: pool.clone(),
                    client: client.clone(),
                })
            })
        })
        .intents(serenity::GatewayIntents::non_privileged())
        .run()
        .await?;
    Ok(())
}
