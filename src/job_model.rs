use serenity::{prelude::Context, model::prelude::Message};

use crate::comment::Comment;

pub struct JobMsg {
    pub str: Option<&'static str>,
    pub job_model: JobModel
}

pub struct JobModel {
    msgs: Vec<Comment>,
    context: Context,
    discord_msg: Message
}