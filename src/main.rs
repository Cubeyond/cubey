mod utils;
mod commands;

use crate::utils::logger;
use crate::commands::skyblock::get_player_uuid;

use poise::serenity_prelude as serenity;

use tracing::error;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    logger::initialise_subscriber();

    let token = match dotenvy::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            error!("Missing DISCORD_TOKEN: {:?}", e);
            return;
        }
    };

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                get_player_uuid::uuid()
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}