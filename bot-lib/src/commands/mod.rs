mod aur_search;
mod bank;
mod cice;
mod class_commands;
mod course_catalog;
mod db_admin;
mod feedback;
mod help;
mod llm_prompt;
mod register;
mod russian_roulette;
mod sathya;
mod set_bot_role;
mod set_dog_role;
mod timeout;
mod yeet;

pub use aur_search::*;
pub use bank::*;
pub use cice::*;
pub use class_commands::*;
pub use course_catalog::*;
pub use db_admin::*;
pub use feedback::*;
pub use help::*;
pub use llm_prompt::*;
pub use register::*;
pub use russian_roulette::*;
pub use sathya::*;
pub use set_bot_role::*;
pub use set_dog_role::*;
pub use timeout::*;
pub use yeet::*;

use crate::data::PoiseContext;
use color_eyre::eyre::{Context, ContextCompat, OptionExt, Result};
use poise::serenity_prelude::{self as serenity, CreateMessage, Member, Mentionable};
use std::time::Duration;

pub async fn get_member(ctx: PoiseContext<'_>) -> Result<Member> {
    let author = ctx.author();
    let guild = ctx.guild().ok_or_eyre("Couldn't get guild")?.id;

    Ok(guild.member(ctx, author.id).await?)
}

pub async fn is_stefan(ctx: PoiseContext<'_>) -> Result<bool> {
    let author = ctx.author();
    let channel_id = ctx.channel_id();
    let guild_id = ctx.guild_id().wrap_err("No guild ID?")?;

    if author.id == 216767618923757568 {
        return Ok(true);
    }

    let timeout_end = chrono::Utc::now() + Duration::from_secs(300);

    if guild_id
        .edit_member(
            ctx,
            author.id,
            serenity::EditMember::new().disable_communication_until(timeout_end.to_rfc3339()),
        )
        .await
        .wrap_err("Failed to edit member")
        .is_err()
    {
        ctx.say("You're a mod why are you trying this? How dare you. You should know better.")
            .await?;
        return Ok(false);
    };

    channel_id
        .send_message(
            &ctx,
            CreateMessage::new().content(format!(
                "{} dared to impersonate Stefan, they were timed out for 5 minutes",
                author.mention()
            )),
        )
        .await?;

    ctx.say("PAH!").await?;

    Ok(false)
}
