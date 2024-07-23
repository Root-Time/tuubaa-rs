use std::string;

use poise::serenity_prelude;
use tracing::info;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, subcommands("test", "name"))]
pub async fn voice(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let ping = ctx.ping().await;
    let response = format!("Pong! {:?}ms", ping.as_millis());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn name(
    ctx: Context<'_>,
    #[description = "Wie willst du den Voice Channel nennen"] name: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    info!("Name: {}", name);
    Ok(())
}
