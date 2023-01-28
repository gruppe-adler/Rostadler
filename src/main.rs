mod commands;
use commands::*;
mod util;
use dotenv::dotenv;
use std::{collections::HashSet, env};
mod events;
use poise::serenity_prelude::{self as serenity};

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let owners_string = env::var("OWNERS").expect("Expected owners in the environment");
    let mut owners: HashSet<serenity::UserId> = HashSet::new();
    for owner in owners_string.split(",") {
        owners.insert(serenity::UserId(
            owner.parse::<u64>().expect("Failed to parse owner"),
        ));
    }

    // Build our client.
    let client = poise::Framework::builder()
        .token(token)
        .intents(
            serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_MESSAGES,
        )
        .options(poise::FrameworkOptions {
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(events::event_listener(_ctx, event, _framework, _data))
            },
            commands: vec![leet::leet(), manage::shutdown(), offtopic::offtopic()],
            owners,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
