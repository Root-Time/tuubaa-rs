use std::sync::atomic::AtomicU32;

use poise::serenity_prelude::{self, ChannelId};
use tracing::warn;

use crate::{
    prisma::{self, PrismaClient},
    voice::Voice,
};

#[derive(Debug)]
pub struct Data {
    pub poise_mentions: AtomicU32,
    pub voice: Voice,
    pub config: Config,
    pub client: PrismaClient,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug)]
pub struct Config {
    pub channel: Channel,
    pub member: Member,
    pub role: Role,
}

impl Config {
    pub async fn new(ctx: &serenity_prelude::Context, client: &PrismaClient) -> Self {
        Config {
            channel: Channel {
                counter: get_channel(ctx, client, "counter").await,
                logs: get_channel(ctx, client, "logs").await,
                welcome: get_channel(ctx, client, "welcome").await,
                rules: get_channel(ctx, client, "rules").await,
                roles: get_channel(ctx, client, "roles").await,
                normal_create: get_channel(ctx, client, "normal_create").await,
            },
            member: Member {},
            role: Role {},
        }
    }
}

#[derive(Debug)]
pub struct Channel {
    pub counter: Option<serenity_prelude::Channel>,
    pub logs: Option<serenity_prelude::Channel>,
    pub welcome: Option<serenity_prelude::Channel>,
    pub rules: Option<serenity_prelude::Channel>,
    pub roles: Option<serenity_prelude::Channel>,
    pub normal_create: Option<serenity_prelude::Channel>,
}

impl Channel {
    pub async fn new(ctx: &serenity_prelude::Context, client: &PrismaClient) -> Self {
        Channel {
            counter: get_channel(ctx, client, "counter").await,
            logs: get_channel(ctx, client, "logs").await,
            welcome: get_channel(ctx, client, "welcome").await,
            rules: get_channel(ctx, client, "rules").await,
            roles: get_channel(ctx, client, "roles").await,
            normal_create: get_channel(ctx, client, "normal_create").await,
        }
    }
}

#[derive(Debug)]
pub struct Member {}

#[derive(Debug)]
pub struct Role {}

async fn get_channel(
    ctx: &serenity_prelude::Context,
    client: &PrismaClient,
    name: &str,
) -> Option<serenity_prelude::Channel> {
    let channel: Option<prisma::config::Data> = client
        .config()
        .find_unique(prisma::config::name::equals(name.to_owned()))
        .exec()
        .await
        .unwrap();

    match channel {
        None => {
            warn!("Channel not found: {}", name);
            return None;
        }
        Some(channel) => ChannelId::new(channel.id as u64).to_channel(ctx).await.ok(),
    }
}
