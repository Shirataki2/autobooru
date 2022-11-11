use poise::FrameworkError;

use crate::data::Data;

#[derive(thiserror::Error, Debug)]
pub enum BotError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serenity error: {0}")]
    Serenity(#[from] poise::serenity_prelude::Error),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub async fn on_error(err: FrameworkError<'_, Data, BotError>) {
    match err {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e)
            }
        }
    }
}
