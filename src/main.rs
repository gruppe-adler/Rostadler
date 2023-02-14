mod commands;
use commands::*;
mod util;
use dotenv::dotenv;
use std::{collections::HashSet, env, sync::Arc};
use tracing::warn;
use util::teamspeak::Ts3Service;

mod events;
use poise::serenity_prelude::{self as serenity};
pub struct DataInner {
    pub ts_client: Arc<Ts3Service>,
}

pub struct Data(pub Arc<DataInner>);
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let owners_string = env::var("OWNERS").expect("Expected owners in the environment");

    let owners = owners_string
        .split(",")
        .into_iter()
        .map(|o| serenity::UserId(o.parse::<u64>().expect("Failed to parse owner")))
        .collect::<HashSet<serenity::UserId>>();

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
            commands: vec![
                leet::leet(),
                manage::shutdown(),
                offtopic::offtopic(),
                teamspeak::ts(),
            ],
            owners,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let ts_client = Arc::new(Ts3Service::new(ctx.clone()).await);
                let ts2 = ts_client.clone();
                tokio::spawn(async move {
                    ts2.start().await.unwrap();
                    tokio::signal::ctrl_c().await.unwrap();
                    ts2.stop().await.unwrap();
                });

                let data = Data(Arc::new(DataInner { ts_client }));

                Ok(data)
            })
        })
        .build()
        .await
        .expect("Error creating client");

    let shard_manager = client.shard_manager().clone();
    tokio::spawn(async move {
        #[cfg(unix)]
        {
            use tokio::signal::unix as signal;

            let [mut s1, mut s2, mut s3] = [
                signal::signal(signal::SignalKind::hangup()).unwrap(),
                signal::signal(signal::SignalKind::interrupt()).unwrap(),
                signal::signal(signal::SignalKind::terminate()).unwrap(),
            ];

            tokio::select!(
                v = s1.recv() => v.unwrap(),
                v = s2.recv() => v.unwrap(),
                v = s3.recv() => v.unwrap(),
            );
        }
        #[cfg(windows)]
        {
            let (mut s1, mut s2) = (
                tokio::signal::windows::ctrl_c().unwrap(),
                tokio::signal::windows::ctrl_break().unwrap(),
            );

            tokio::select!(
                v = s1.recv() => v.unwrap(),
                v = s2.recv() => v.unwrap(),
            );
        }

        warn!("Recieved control C and shutting down.");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}
