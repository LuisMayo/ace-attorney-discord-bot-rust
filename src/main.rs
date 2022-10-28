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

static QUEUED_IDS: std::sync::RwLock<Vec<(u64, u64)>> = std::sync::RwLock::new(Vec::new());
#[group]
#[commands(ping)]
#[commands(invite)]
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
#[min_args(1)]
#[max_args(2)]
async fn ping(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if (!args.current().is_some()) {
        msg.reply(ctx, "Sorry, you need to provide at least one argument");
    } else {
        let number = args.parse::<u8>().unwrap_or(5);
        let my_msg = msg.to_owned();
        let my_ctx = ctx.to_owned();
        tokio_rayon::spawn(move || async {
            let foo = msg.channel(ctx).await?.guild();
        });
    }
    msg.reply(ctx, "Pong!").await?;
    if foo.is_some(){
        let guild = foo.unwrap();
        let first_msg = if msg.referenced_message.is_some() {msg.referenced_message.as_ref().unwrap()} else {msg};
        let messages = guild.messages(ctx, |retriever| retriever.before(first_msg).limit(1)).await?;
        let internal_msgs = messages.iter().map(|msg| Comment::new(&msg)).collect();
        render_comment_list(&internal_msgs);
    }
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