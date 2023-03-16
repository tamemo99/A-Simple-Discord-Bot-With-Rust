mod commands;

use colored::Colorize;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use std::env;
use tracing::info;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::commands::stock::*;
use crate::commands::maher::*;
use crate::commands::happy_birthday::*;
use crate::commands::christos::*;

const HELP_MESSAGE: &str = "
    Bot Supports following commands:
    !help
    !stock
    ... more stuff blah blah blah
";

const HELP_COMMAND: &str = "!help";

#[group]
#[commands(stock,maher, happy_birthday,christos)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("{} {:?}", "Error sending message: ".red(), why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    //Use the token of the bot
    let token = env::var("DISCORD_TOKEN").expect("Expted a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);


    let intents = GatewayIntents::GUILD_MESSAGES //enables bot to send messages
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    println!("{}", "Hello, world!".magenta());
}
