use std::env;
use std::sync::Arc;

use regex::Regex;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use serenity::model::prelude::Message;
use serenity::model::webhook::Webhook;
use serenity::prelude::Context;

pub async fn twitter_link_replace(ctx: Context, msg: Message) {
    let re = Regex::new(r"https://twitter.com").unwrap();
    if re.is_match(msg.content.as_str()) {
        let author_name = msg.author.name.clone();
        let avatar_url = msg.author.avatar_url().unwrap();
        let new_content = re.replace(&msg.content, "https://vxtwitter.com");
        let http =
            Http::new(&env::var("DISCORD_TOKEN").expect("Expected TWITTER_WEBHOOK in environment"));
        let webhooks = ChannelId::webhooks(msg.channel_id, &http)
            .await
            .expect("Failed to fetch webhooks");
        let ctx2 = ctx.clone();
        let found_webhook = async {
            let found_webhook = webhooks.into_iter().find(|hook| {
                return match hook.channel_id {
                    Some(id) => id == msg.channel_id,
                    _ => false,
                };
            });
            match found_webhook {
                Some(hook) => hook,
                _ => {
                    return ChannelId::create_webhook(&msg.channel_id, ctx, msg.channel_id)
                        .await
                        .expect("Something went wrong when creating new webhook");
                }
            }
        };

        msg.delete(ctx2).await.unwrap();
        let webhook = Webhook::from_url(&http, found_webhook.await.url().unwrap().as_str())
            .await
            .expect("Webhook error");
        webhook
            .execute(&http, false, |w| {
                w.content(new_content)
                    .username(author_name)
                    .avatar_url(avatar_url)
            })
            .await
            .expect("Could not execute webhook.");
    }
}