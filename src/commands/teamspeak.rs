use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn ts(ctx: Context<'_>) -> Result<(), Error> {
    let clients = ctx.data().0.ts_client.users.lock().await;
    let test2 = clients
        .iter()
        .map(|c| c.1.client_nickname.as_str())
        .collect::<Vec<&str>>()
        .join(", ");
    ctx.say(test2).await?;
    Ok(())
}
