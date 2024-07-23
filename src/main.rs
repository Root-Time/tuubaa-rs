use poise::serenity_prelude::{self as serenity, futures::stream::FuturesUnordered, GuildId};

use dotenvy::dotenv;
use prisma::PrismaClient;
use std::{
    env,
    sync::atomic::{AtomicU32, Ordering},
};
use tracing::info;

#[allow(warnings, unused)]
mod prisma;

mod modules;
pub use modules::*;

mod model;
pub use model::{Context, Data, Error};

pub mod utils;

#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
    // #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    // let u = user.as_ref().unwrap_or_else(|| ctx.author());
    // let response = format!("{}'s account was created at {}", u.name, u.created_at());
    // ctx.say(response).await?;
    let ping = ctx.ping().await;
    let response = format!("Pong! {:?}ms", ping.as_millis());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    // let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");

    let client = PrismaClient::_builder().build().await?;

    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // Commands
            commands: vec![ping(), voice::voice()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },

            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                if args.len() > 1 && args[1] == "reload-commands" {
                    info!("Reloading commands");
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        GuildId::new(1244082140140802162),
                    )
                    .await?;
                }
                // else {
                //     poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                // }

                let data = Data {
                    voice: voice::Voice::new(&client),
                    config: model::Config::new(ctx, &client).await,
                    poise_mentions: AtomicU32::new(0),
                    client,
                };

                Ok(data)
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
            let modules = FuturesUnordered::new();
            modules.push(voice::entry(ctx, data));
            modules.push(ticket::entry(ctx, data));
        }
        serenity::FullEvent::VoiceChannelStatusUpdate {
            old,
            status,
            id,
            guild_id,
        } => {
            info!(
                "Voice channel status update: {:?} {:?} {:?} {:?}",
                old, status, id, guild_id
            );
        }
        serenity::FullEvent::VoiceServerUpdate { event } => {
            info!("Voice server update: {:?}", event);
        }
        serenity::FullEvent::VoiceStateUpdate { old, new } => {
            info!("TESTING");
            tokio::try_join!(
                voice::events::on_delete(ctx, old, new, data),
                voice::events::on_create(ctx, old, new, data),
            )?;
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.content.to_lowercase().contains("poise")
                && new_message.author.id != ctx.cache.current_user().id
            {
                let old_mentions = data.poise_mentions.fetch_add(1, Ordering::SeqCst);
                new_message
                    .reply(
                        ctx,
                        format!("Poise has been mentioned {} times", old_mentions + 1),
                    )
                    .await?;
            }
        }
        _ => {}
    }
    Ok(())
}
