use poise::serenity_prelude as serenity;

use dotenvy;
use tokio::time::Instant;
use tracing::{error, info, level_filters::LevelFilter, subscriber, warn};
use tracing_subscriber::{fmt::Subscriber, EnvFilter};


struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    initialise_subscriber();

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
            commands: vec![age()],
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

fn initialise_subscriber() {
    let start_time = Instant::now();

    tracing_subscriber::fmt::init();

    let rust_log = match dotenvy::var("RUST_LOG") {
        Ok(level) => level,
        Err(_) => {
            error!("Could not get log level from .env file, setting default to INFO ...");
            format!("info")
        }
    };

    let filter = match EnvFilter::try_new(format!("cubey={rust_log}")) {
        Ok(filter) => filter,
        Err(_) => {
            error!("Could not get filter from .env file, setting default...");
            EnvFilter::default()
        }
    };

    let subscriber = Subscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .with_env_filter(filter)
        .compact()
        .finish();

    if let Err(_) = subscriber::set_global_default(subscriber) {
        warn!("Could not set custom subscriber, setting default...");

        let default_subscriber = Subscriber::default();
        let _ = subscriber::set_global_default(default_subscriber);
    }

    let elapsed_time = start_time.elapsed();
    info!("Initalised subscriber in {elapsed_time:.2?}")
}