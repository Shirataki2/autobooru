use poise::{
    serenity_prelude::{Context, MessageComponentInteraction},
    Event, FrameworkContext,
};

use crate::{data::Data, error::BotError, task::spawn_tasks};

pub async fn handle_event(
    ctx: &Context,
    ev: &Event<'_>,
    _fw: FrameworkContext<'_, Data, BotError>,
    data: &Data,
) -> Result<(), BotError> {
    #[allow(clippy::single_match)]
    match ev {
        Event::InteractionCreate { interaction } => {
            handle_interaction(ctx, interaction).await?;
        }
        Event::CacheReady { .. } => {
            spawn_tasks(ctx.clone(), data.clone()).await;
        }
        _ => {}
    }
    Ok(())
}

enum Interaction {
    Test(Box<MessageComponentInteraction>),
    Unknown,
}

impl From<&poise::serenity_prelude::Interaction> for Interaction {
    fn from(interaction: &poise::serenity_prelude::Interaction) -> Self {
        match interaction {
            poise::serenity_prelude::Interaction::MessageComponent(comp) => {
                match comp.data.custom_id.as_str() {
                    "test" => Interaction::Test(Box::new(comp.clone())),
                    _ => Interaction::Unknown,
                }
            }
            _ => Interaction::Unknown,
        }
    }
}

async fn handle_interaction(
    ctx: &Context,
    interaction: &poise::serenity_prelude::Interaction,
) -> Result<(), BotError> {
    debug!("recieve interaction: {:?}", interaction);
    match Interaction::from(interaction) {
        Interaction::Test(command) => {
            command
                .create_interaction_response(&ctx.http, |r| {
                    r.kind(
                        poise::serenity_prelude::InteractionResponseType::ChannelMessageWithSource,
                    );
                    r.interaction_response_data(|d| {
                        d.content("test");
                        d
                    });
                    r
                })
                .await?;
        }
        Interaction::Unknown => {}
    }
    Ok(())
}
