use poise::serenity_prelude::Context;
use tracing::info;

use crate::Data;

pub fn entry(_ctx: &Context, _data: &Data) {
    info!("Ticket module loaded!");
}
