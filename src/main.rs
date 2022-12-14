use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

use crate::comment::Comment;
use crate::job_model::JobMsg;
use crate::obj_engine_handler::MySender;
use crate::obj_engine_handler::init_python;


mod comment;
mod obj_engine_handler;
mod config;
mod job_model;

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
    let data_sender = init_python();
    data_lock.insert::<config::Settings>(config);
    data_lock.insert::<MySender<JobMsg>>(data_sender);
    drop(data_lock);

    // start listening for events by starting a single shard
    if let Err(why) = client.start_autosharded().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn render(ctx: &Context, msg: &Message) -> CommandResult {
    // let number = args.parse::<u8>().unwrap_or(5);
    let number_of_msgs_unparsed: String =  msg.content.chars().skip(7).collect();
    let number = number_of_msgs_unparsed.trim().parse::<u8>().unwrap_or(5);
    println!("Ping");
    let my_msg = msg.to_owned();
    let my_ctx = ctx.to_owned();
    println!("Pong");
    let guild_response = my_msg.channel(&my_ctx).await;
    if guild_response.is_err(){
        return Ok(());
    }
    let guild = guild_response.unwrap().guild();
    if guild.is_some(){
        println!("Yay!");
        let channel = guild.unwrap();
        let first_msg = if my_msg.referenced_message.is_some() {my_msg.referenced_message.as_ref().unwrap()} else {&my_msg};
        let messages_result = channel.messages(&my_ctx, |retriever| retriever.before(first_msg).limit(number.into())).await;
        if messages_result.is_ok() {
            println!("yey!");
            let messages = messages_result.unwrap();
            let comments: Vec<Comment> = messages.iter().map(|msg| Comment::new(&msg)).into_iter().collect();
            let lock = ctx.data.read().await;
            let sender = lock.get::<MySender<JobMsg>>().unwrap();
            sender.0.send(JobMsg { str: None, job_model: job_model::JobModel { msgs: comments, context: my_ctx, discord_msg: my_msg } });
            return Ok(());
            // let internal_msgs: Vec<Comment> = messages.par_iter().map(|msg| Comment::new(&msg)).into_par_iter().collect();
            // let result = tokio_rayon::spawn(move || render_comment_list(&internal_msgs, my_msg.id.0)).await;
        } else {
            println!("sad!");
            let _sorry_msg = my_msg.reply(&my_ctx, "Sorry I couldn't retrieve the messages").await;
        }
    }
    msg.reply(ctx, "Pong!").await;
    return Ok(());
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