use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

use crate::comment::Comment;
use crate::obj_engine_handler::render_comment_list;


mod comment;
mod obj_engine_handler;
#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token triste");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start_autosharded().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    let foo = msg.channel(ctx).await?.guild();
    if foo.is_some(){
        let guild = foo.unwrap();
        let first_msg = if msg.referenced_message.is_some() {msg.referenced_message.as_ref().unwrap()} else {msg};
        let messages = guild.messages(ctx, |retriever| retriever.before(first_msg).limit(1)).await?;
        let internal_msgs = messages.iter().map(|msg| Comment::new(&msg)).collect();
        render_comment_list(&internal_msgs);
    }
    Ok(())
}