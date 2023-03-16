use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn maher(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "<@485521970302156800> You are such a nice man")
        .await?;

    Ok(())
}