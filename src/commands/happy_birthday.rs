use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn happy_birthday(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, format!("Happy Birthday 🥳 {}", args.message()))
        .await?;

    Ok(())
}
