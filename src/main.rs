use serenity::async_trait;
use serenity::framework::standard::Args;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

use crate::comment::Comment;
use crate::obj_engine_handler::render_comment_list;


mod comment;
mod obj_engine_handler;
mod config;

// static QUEUED_IDS: std::sync::RwLock<Vec<(u64, u64)>> = std::sync::RwLock::new(Vec::new());
#[group]
#[commands(render, invite)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    // queue: Vec<Message> = Mutex::new(Vec::new());
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);
    let config = config::Settings::new();
    println!("{}", &config.token);
    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(&config.token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let mut data_lock = client.data.write().await;
    data_lock.insert::<config::Settings>(config);
    drop(data_lock);

    // start listening for events by starting a single shard
    if let Err(why) = client.start_autosharded().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn render(ctx: &Context, msg: &Message) -> CommandResult {
    // let number = args.parse::<u8>().unwrap_or(5);
    println!("Ping");
    let my_msg = msg.to_owned();
    let my_ctx = ctx.to_owned();
    println!("Pong");
    let _thread_result = tokio::task::spawn(async move {
        println!("I'm on the thread hjehe");
        let guild_response = my_msg.channel(&my_ctx).await;
        if guild_response.is_err(){
            return;
        }
        let guild = guild_response.unwrap().guild();
        if guild.is_some(){
            println!("Yay!");
            let channel = guild.unwrap();
            let first_msg = if my_msg.referenced_message.is_some() {my_msg.referenced_message.as_ref().unwrap()} else {&my_msg};
            let messages_result = channel.messages(&my_ctx, |retriever| retriever.before(first_msg).limit(1)).await;
            if messages_result.is_ok() {
                println!("yey!");
                let messages = messages_result.unwrap();
                let internal_msgs = messages.iter().map(|msg| Comment::new(&msg)).collect();
                render_comment_list(&internal_msgs);
            } else {
                println!("sad!");
                let _sorry_msg = my_msg.reply(&my_ctx, "Sorry I couldn't retrieve the messages").await;
            }
        }
    });
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    let lock = ctx.data.read().await;
    let settings_opt = lock.get::<config::Settings>();
    if settings_opt.is_some() {
        msg.reply(ctx, &settings_opt.unwrap().invite_link).await.unwrap();
    } else {
        msg.reply(ctx, "I can't send you the link :(").await.unwrap();
    }
    Ok(())
}