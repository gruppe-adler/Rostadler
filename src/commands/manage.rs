use crate::{Context, Error};

#[poise::command(slash_command, owners_only)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}

// #[poise::command(slash_command, owners_only)]
// pub async fn restart(ctx: Context<'_>) -> Result<(), Error> {
//     ctx.say("Restarting").await?;
//     ctx.framework()
//         .shard_manager()
//         .lock()
//         .await
//         .restart(poise::serenity_prelude::ShardId(0))
//         .await;
//     ctx.framework()
//         .shard_manager()
//         .lock()
//         .await
//         .restart(poise::serenity_prelude::ShardId(1))
//         .await;
//     Ok(())
// }
