use crate::{util::twitter::twitter_link_replace, Data, Error};
use poise::serenity_prelude::Context;
use poise::serenity_prelude::FullEvent;
//Event nonsense
pub async fn event_listener(
    ctx: &Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            ready(data_about_bot, ctx, _framework).await?;
        }
        FullEvent::Message { new_message } => twitter_link_replace(ctx, new_message).await,
        _ => {
            println!("{}", event.snake_case_name());
        }
    }

    Ok(())
}
//ready
async fn ready(
    data: &poise::serenity_prelude::Ready,
    _ctx: &Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    println!("{} is connected!", data.user.name);

    // let ts_client = Ts3Service::new().await;
    Ok(())
}
