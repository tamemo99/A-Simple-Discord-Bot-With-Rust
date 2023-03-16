use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn christos(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "<@341667252640153601> STOP working on the code behind my back!")
        .await?;

    Ok(())
}