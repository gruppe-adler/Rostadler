use crate::{Context, Error};
use poise::serenity_prelude::{
    ChannelId, GuildChannel, Http, PermissionOverwrite, PermissionOverwriteType, Permissions,
    RoleId,
};
use std::env;

#[poise::command(slash_command, subcommands("create", "archive", "unarchive", "order"))]
pub async fn offtopic(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[derive(Debug, poise::Modal)]
#[allow(dead_code)] // fields only used for Debug print
struct CreateOfftopicChannelModal {
    #[name = "Channel Name"]
    #[placeholder = "r6-siege"]
    #[min_length = 5]
    #[max_length = 100]
    channel_name: String,

    #[name = "Role Name"]
    #[placeholder = "R6Siege"]
    role_name: Option<String>,

    #[name = "Description"]
    #[placeholder = "Cool Channel"]
    #[paragraph]
    description: Option<String>,
}

#[poise::command(slash_command)]
pub async fn create(
    ctx: poise::ApplicationContext<'_, crate::Data, crate::Error>,
) -> Result<(), Error> {
    use poise::Modal as _;

    if let Some(data) = CreateOfftopicChannelModal::execute(ctx).await? {
        ctx.defer_ephemeral().await?;
        let offtopic_category_id = env::var("OFFTOPIC_CATEGORY_ID")
            .expect("Expected OFFTOPIC_CATEGORY_ID in the environment");

        // TODO check if channel already exists

        let http =
            Http::new(&env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment"));

        if let Some(guild_id) = ctx.guild_id() {
            let guild = http.get_guild(guild_id.0).await?;
            let permission_name = match data.role_name {
                Some(role_name) => role_name,
                None => data.channel_name.clone(),
            };

            let topic = match data.description {
                Some(description) => description,
                None => "".to_string(),
            };

            let role = guild
                .create_role(&http, |role| role.name(permission_name))
                .await?;

            guild
                .create_channel(&http, |channel| {
                    channel
                        .name(data.channel_name)
                        .category(
                            offtopic_category_id
                                .parse::<u64>()
                                .expect("Failed to parse offtopic category id"),
                        )
                        .topic(topic)
                        .permissions(vec![
                            PermissionOverwrite {
                                allow: Permissions::VIEW_CHANNEL,
                                deny: Permissions::empty(),
                                kind: PermissionOverwriteType::Role(role.id),
                            },
                            PermissionOverwrite {
                                allow: Permissions::empty(),
                                deny: Permissions::VIEW_CHANNEL,
                                kind: PermissionOverwriteType::Role(RoleId(guild_id.0)),
                            },
                        ])
                })
                .await?;

            ctx.say("Viel Spaß mit deinem Channel!").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn archive(
    _ctx: Context<'_>,
    #[description = "The channel that you want to archive"] mut channel: GuildChannel,
) -> Result<(), Error> {
    let http =
        Http::new(&env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment"));
    let offtopic_archive_category_id = env::var("OFFTOPIC_ARCHIVE_CATEGORY_ID")
        .expect("Expected OFFTOPIC_ARCHIVE_CATEGORY_ID in the environment")
        .parse::<u64>()
        .expect("Failed to parse OFFTOPIC_ARCHIVE_CATEGORY_ID to u64");

    // TODO check if channel is already in the archive / check if channel is offtopic channel

    channel
        .edit(http, |c| {
            c.category(ChannelId(offtopic_archive_category_id))
        })
        .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn unarchive(
    _ctx: Context<'_>,
    #[description = "The channel that you want to unarchive"] mut channel: GuildChannel,
) -> Result<(), Error> {
    let http =
        Http::new(&env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment"));
    let offtopic_category_id = env::var("OFFTOPIC_CATEGORY_ID")
        .expect("Expected OFFTOPIC_CATEGORY_ID in the environment")
        .parse::<u64>()
        .expect("Failed to parse OFFTOPIC_CATEGORY_ID to u64");

    // TODO check if channel is currently archived

    channel
        .edit(http, |c| c.category(ChannelId(offtopic_category_id)))
        .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn order(ctx: Context<'_>) -> Result<(), Error> {
    let offtopic_category_id = env::var("OFFTOPIC_CATEGORY_ID")
        .expect("Expected OFFTOPIC_CATEGORY_ID in the environment")
        .parse::<u64>()
        .expect("Failed to parse OFFTOPIC_CATEGORY_ID to u64");

    let http =
        Http::new(&env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment"));
    let mut offtopic_channels = Vec::new();
    if let Some(guild_id) = ctx.guild_id() {
        for channel in guild_id.channels(&http).await? {
            if let Some(channel_parent_id) = channel.1.parent_id {
                if channel_parent_id.0 == offtopic_category_id {
                    offtopic_channels.push(channel);
                }
            }
        }
        offtopic_channels.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        for (pos, mut channel) in offtopic_channels.into_iter().enumerate() {
            channel
                .1
                .edit(&http, |c| {
                    c.position(pos.try_into().expect("Failed to convert channel index"))
                })
                .await?;
        }
    }

    Ok(())
}
