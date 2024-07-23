use std::{collections::HashMap, sync::Arc};

use poise::serenity_prelude::{ChannelId, Context, UserId};

mod commands;
pub use commands::*;
use tokio::sync::Mutex;
use tracing::info;

pub mod events;

use crate::{prisma::PrismaClient, Data};

pub fn entry(_ctx: &Context, _data: &Data) {
    info!("Voice module loaded!");
}

#[derive(Clone, Debug, Default)]
pub struct Voice {
    pub users: Arc<Mutex<HashMap<UserId, User>>>,
    pub channels: Arc<Mutex<HashMap<ChannelId, Channel>>>,
}

#[derive(Clone, Debug, Default)]
pub struct User {
    pub user_id: UserId,
    pub owner_of: Vec<ChannelId>,
    pub friends: Vec<UserId>,
    pub blocked: Vec<UserId>,
}

#[derive(Clone, Debug, Default)]
pub struct Channel {
    pub channel_id: ChannelId,
    pub owner: UserId,
    pub members: Vec<UserId>,
    pub private: bool,
}

impl Voice {
    pub fn new(_client: &PrismaClient) -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
