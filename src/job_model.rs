use crossbeam_channel::{Sender, Receiver};
use serenity::{prelude::Context, model::prelude::Message};

use crate::comment::Comment;

pub struct JobMsg {
    pub str: Option<&'static str>,
    pub job_model: JobModel
}

pub struct JobModel {
    pub msgs: Vec<Comment>,
    pub context: Context,
    pub discord_msg: Message
}