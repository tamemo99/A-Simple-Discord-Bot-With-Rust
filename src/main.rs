use colored::Colorize;
use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

const HELP_MESSAGE: &str = "
    Bot Supports following commands:
    !help
    !stock
    ... more stuff blah blah blah
";

const HELP_COMMAND: &str = "!help";

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
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expted a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
