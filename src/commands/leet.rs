use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use std::env;

#[poise::command(context_menu_command = "Leet")]
pub async fn leet(
    _ctx: Context<'_>,
    #[description = "Discord profile to query information about"] message: serenity::Message,
) -> Result<(), Error> {
    let http = serenity::Http::new(
        &env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment"),
    );
    message
        .reply(
            http,
            format!(
                "<t:{}>",
                (*message.id.as_u64() / 4194304 + 1420070400000) / 1000
            ),
        )
        .await?;
    _ctx.defer_ephemeral().await?;
    _ctx.say("Executed command".to_string()).await?;
    Ok(())
}
