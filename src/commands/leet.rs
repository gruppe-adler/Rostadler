use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CacheHttp};

#[poise::command(context_menu_command = "Leet")]
pub async fn leet(
    ctx: Context<'_>,
    #[description = "Discord profile to query information about"] message: serenity::Message,
) -> Result<(), Error> {
    message
        .reply(
            ctx.http(),
            format!(
                "<t:{}>",
                (*message.id.as_u64() / 4194304 + 1420070400000) / 1000
            ),
        )
        .await?;
    ctx.defer_ephemeral().await?;
    ctx.say("Executed command".to_string()).await?;
    Ok(())
}
