use crate::{util::twitter::twitter_link_replace, Data, Error};
use poise::serenity_prelude;
//Event nonsense
pub async fn event_listener(
    ctx: &serenity_prelude::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            ready(data_about_bot, ctx, _framework).await?;
        }
        poise::Event::Message { new_message } => twitter_link_replace(ctx, new_message).await,
        _ => {
            println!("{}", event.name());
        }
    }

    Ok(())
}
//ready
async fn ready(
    data: &poise::serenity_prelude::Ready,
    _ctx: &serenity_prelude::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    println!("{} is connected!", data.user.name);
    Ok(())
}
