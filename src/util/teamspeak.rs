use async_trait::async_trait;
use chrono::Utc;
use dotenv::dotenv;
use poise::serenity_prelude::{user, CacheHttp, ChannelId, Context};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;
use ts3::{
    event::{ClientEnterView, ClientLeftView},
    Client, CommandBuilder, Decode, EventHandler,
};

use crate::Error;

#[derive(Default, Decode, Debug, Clone)]
pub struct TSUser {
    pub client_database_id: u32,
    pub client_lastconnected: String,
    pub client_nickname: String,
    pub client_idle_time: String,
    pub client_platform: String,
    pub client_type: String,
    pub cid: u16,
    pub clid: usize,
}

impl TSUser {
    pub async fn is_new_user(&self) -> Result<bool, Error> {
        let now = Utc::now().timestamp();
        let date = self.client_lastconnected.parse::<i64>().unwrap();
        return Ok((now - date) < 3000);
    }
}

#[derive(Default, Decode, Debug)]
pub struct TSChannel {
    pub cid: u16,
    pub channel_name: String,
    pub total_clients: u16,
}

pub struct Handler {
    discord_ctx: Context,
    users: Arc<Mutex<HashMap<usize, TSUser>>>,
    channels: Arc<Mutex<HashMap<u16, TSChannel>>>,
    service: Ts3Service,
}

#[async_trait]
impl EventHandler for Handler {
    async fn cliententerview(&self, _client: Client, event: ClientEnterView) {
        dotenv().ok();
        // println!("Client {:?} joined!", event);
        let mut users = self.users.lock().await;
        let user = self.service.get_client(event.clid).await.unwrap();
        users.insert(user.clid, user.clone());
        // users.insert(event.clid, v)
        if event.client_type != 1 {
            let channel = self
                .discord_ctx
                .http()
                .get_channel(
                    env::var("TS3_CHANNEL")
                        .expect("Expected TS3_HOST in the environment")
                        .parse::<u64>()
                        .unwrap(),
                )
                .await
                .unwrap();
            // let emoji_join =
            //     env::var("TS3_JOIN_EMOJI").expect("Expected TS3_JOIN_EMOJI in the environment");
            // println!(
            //     "{}",
            //     format!(":{}: **{}** joined", emoji_join, event.client_nickname)
            // );
            if let Some(guild_channel) = channel.guild() {
                let emoji_join =
                    env::var("TS3_JOIN_EMOJI").expect("Expected TS3_JOIN_EMOJI in the environment");

                guild_channel
                    .send_message(self.discord_ctx.http(), |m| {
                        m.content(format!(
                            ":{}: **{}** joined",
                            emoji_join, event.client_nickname
                        ))
                    })
                    .await
                    .unwrap();

                if user.is_new_user().await.unwrap() {
                    guild_channel
                        .send_message(self.discord_ctx.http(), |m| {
                            m.content(format!("@here {} joined", event.client_nickname))
                        })
                        .await
                        .unwrap();
                }
            }
        }
    }
    async fn clientleftview(&self, _client: Client, event: ClientLeftView) {
        // println!("Client {:?} left!", event);
        dotenv().ok();
        let channel = self
            .discord_ctx
            .http()
            .get_channel(
                env::var("TS3_CHANNEL")
                    .expect("Expected TS3_HOST in the environment")
                    .parse::<u64>()
                    .unwrap(),
            )
            .await
            .unwrap();

        let mut users = self.users.lock().await;

        // if let Some(user_resolved) = users.get(&event.clid) {
        //     if let Some(guild_channel) = channel.guild() {
        //         let emoji_leave = env::var("TS3_LEAVE_EMOJI")
        //             .expect("Expected TS3_LEAVE_EMOJI in the environment");

        //         guild_channel
        //             .send_message(self.discord_ctx.http(), |m| {
        //                 m.content(format!(
        //                     ":{:?}: **{:?}** left",
        //                     emoji_leave, user_resolved.client_nickname
        //                 ))
        //             })
        //             .await
        //             .unwrap();
        //         users.remove(&event.clid);
        //     }
        // }
    }
}
#[derive(Clone)]
pub struct Ts3Service {
    pub client: ts3::Client,
    discord_ctx: Context,
    pub users: Arc<Mutex<HashMap<usize, TSUser>>>,
    pub channels: Arc<Mutex<HashMap<u16, TSChannel>>>,
}

impl Ts3Service {
    pub async fn new(discord_ctx: Context) -> Self {
        dotenv().ok();
        let ts3_host = env::var("TS3_HOST").expect("Expected TS3_HOST in the environment");
        let client = Client::new(ts3_host).await.unwrap();
        let users = Arc::new(Mutex::new(HashMap::new()));
        let channels = Arc::new(Mutex::new(HashMap::new()));
        Self {
            client,
            discord_ctx,
            users,
            channels,
        }
    }

    pub async fn start(&self) -> Result<(), Error> {
        dotenv().ok();
        let ts3_user = env::var("TS3_USER").expect("Expected TS3_USER in the environment");
        let ts3_password =
            env::var("TS3_PASSWORD").expect("Expected TS3_PASSWORD in the environment");
        let ts3_sid = env::var("TS3_SID").expect("Expected TS3_SID in the environment");

        self.client
            .use_sid(ts3_sid.parse::<usize>().unwrap())
            .await
            .unwrap();
        self.client.login(&ts3_user, &ts3_password).await.unwrap();
        self.client.set_event_handler(Handler {
            discord_ctx: self.discord_ctx.clone(),
            users: self.users.clone(),
            channels: self.channels.clone(),
            service: self.clone(),
        });
        self.client
            .servernotifyregister(ts3::client::ServerNotifyRegister::Server)
            .await
            .unwrap();

        let users = self.get_clients().await?;
        let mut user_map = self.users.lock().await;
        for user in users {
            user_map.insert(user.clid, user);
        }
        drop(user_map);
        let channels = self.get_channels().await?;
        let mut channel_map = self.channels.lock().await;
        for channel in channels {
            channel_map.insert(channel.cid, channel);
        }
        drop(channel_map);
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Error> {
        self.client.logout().await?;
        self.client.quit().await?;
        Ok(())
    }

    pub async fn get_clients(&self) -> Result<Vec<TSUser>, Error> {
        let cmd = CommandBuilder::new("clientlist")
            .arg("-info", "")
            .arg("-times", "")
            .into_inner();
        let clients: Vec<TSUser> = self.client.send(cmd).await?;
        // dbg!(&clients);
        Ok(clients)
    }

    pub async fn get_channels(&self) -> Result<Vec<TSChannel>, Error> {
        let channels: Vec<TSChannel> = self
            .client
            .send(CommandBuilder::new("channellist").into_inner())
            .await?;
        // dbg!(channels);
        Ok(channels)
    }

    pub async fn get_client(&self, clid: u64) -> Result<TSUser, Error> {
        let cmd = CommandBuilder::new("clientinfo")
            .arg("clid", clid)
            .into_inner();
        let user: TSUser = self.client.send(cmd).await?;
        Ok(user)
    }
}
