use poise::serenity_prelude::{ChannelType, Context, CreateChannel, VoiceState};
use tracing::{info, warn};

use crate::{
    prisma::{
        self,
        user_config::{self},
    },
    Data, Error,
};

use crate::voice;

pub async fn on_create(
    ctx: &Context,
    _: &Option<VoiceState>,
    new: &VoiceState,
    data: &Data,
) -> Result<(), Error> {
    let (Some(channel_id), Some(normal_create)) =
        (new.channel_id, data.config.channel.normal_create.as_ref())
    else {
        return Ok(());
    };

    if channel_id != normal_create.id() {
        return Ok(());
    }

    let Some(channel) = channel_id.to_channel(ctx).await?.guild() else {
        warn!("Member was none!");
        return Ok(());
    };

    let category_id = channel.parent_id;
    let guild_id = channel.guild_id;
    let Some(member) = new.member.as_ref() else {
        warn!("Member was none!");
        return Ok(());
    };

    let user_config = data
        .client
        .user_config()
        .find_unique(user_config::UniqueWhereParam::UserIdEquals(
            new.user_id.get() as i32,
        ))
        .exec()
        .await?;

    let prefix = user_config
        .map(|x| x.prefix)
        .unwrap_or_else(|| format!(">> {}'s channel", member.display_name()));

    let mut create_channel = CreateChannel::new(prefix).kind(ChannelType::Voice);
    if let Some(category_id) = category_id {
        create_channel = create_channel.category(category_id);
    }

    let channel = guild_id.create_channel(ctx, create_channel).await?;

    member.move_to_voice_channel(ctx, &channel).await?;

    let user_data_lock = data.voice.users.lock().await;
    let prev_user_data = user_data_lock.get(&member.user.id);

    let user_data = prev_user_data.map_or_else(
        || voice::User {
            user_id: member.user.id,
            owner_of: vec![channel.id],
            ..Default::default()
        },
        |prev| voice::User {
            owner_of: vec![channel.id],
            user_id: member.user.id,
            ..prev.clone()
        },
    );

    drop(user_data_lock);

    data.voice
        .users
        .lock()
        .await
        .insert(member.user.id, user_data);

    let channel_data = voice::Channel {
        channel_id: channel.id,
        owner: member.user.id,
        members: Default::default(),
        private: false,
    };

    let mut lock = data.voice.channels.lock().await;

    lock.insert(channel.id, channel_data);

    drop(lock);

    info!("{:?}", channel.id);

    data.client
        .voice()
        .create(
            channel.id.get() as i32,
            member.user.id.get() as i32,
            Vec::new(),
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn on_delete(
    ctx: &Context,
    old: &Option<VoiceState>,
    _: &VoiceState,
    data: &Data,
) -> Result<(), Error> {
    let Some(old) = old else {
        return Ok(());
    };

    let Some(channel_id) = old.channel_id else {
        return Ok(());
    };

    let channel: Vec<u64> = data
        .voice
        .channels
        .lock()
        .await
        .clone()
        .into_iter()
        .map(|(_id, x)| x.channel_id.get())
        .collect();

    if !channel.contains(&channel_id.get()) {
        return Ok(());
    }

    let Some(channel) = channel_id.to_channel(ctx).await?.guild() else {
        return Ok(());
    };

    let Some(count) = channel.members(ctx).ok().map(|x| x.len()) else {
        return Ok(());
    };

    if count > 0 {
        return Ok(());
    }

    channel.delete(ctx).await?;

    data.voice.channels.lock().await.remove(&channel_id);

    data.client
        .voice()
        .delete(prisma::voice::UniqueWhereParam::VoiceIdEquals(
            channel_id.get() as i32,
        ))
        .exec()
        .await?;

    Ok(())
}
