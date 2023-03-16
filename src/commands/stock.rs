use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn stock(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Why you fucking Stupid??")
        .await?;

    Ok(())
}