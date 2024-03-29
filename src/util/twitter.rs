use regex::Regex;
use serenity::all::CreateWebhook;
use serenity::all::ExecuteWebhook;
use serenity::model::prelude::ChannelId;
use serenity::model::prelude::Message;
use serenity::model::webhook::Webhook;
use serenity::prelude::Context;

pub async fn twitter_link_replace(ctx: &Context, msg: &Message) {
    let re = Regex::new(r"https://twitter.com").unwrap();
    if !msg.author.bot && re.is_match(msg.content.as_str()) {
        let author_name = msg.author.name.clone();
        let avatar_url = msg.author.avatar_url().unwrap();
        let new_content = re.replace(&msg.content, "https://vxtwitter.com");
        let webhooks = ChannelId::webhooks(msg.channel_id, &ctx.http)
            .await
            .expect("Failed to fetch webhooks");
        let found_webhook = async {
            let found_webhook = webhooks.into_iter().find(|hook| match hook.channel_id {
                Some(id) => id == msg.channel_id,
                _ => false,
            });
            match found_webhook {
                Some(hook) => hook,
                _ => {
                    let w = CreateWebhook::new(msg.channel_id.get().to_string());
                    ChannelId::create_webhook(msg.channel_id, &ctx, w)
                        .await
                        .expect("Something went wrong when creating new webhook")
                }
            }
        };

        msg.delete(&ctx).await.unwrap();
        let webhook = Webhook::from_url(&ctx.http, found_webhook.await.url().unwrap().as_str())
            .await
            .expect("Webhook error");
        let w = ExecuteWebhook::new()
            .content(new_content)
            .username(author_name)
            .avatar_url(avatar_url);
        webhook
            .execute(&ctx.http, false, w)
            .await
            .expect("Could not execute webhook.");
    }
}
