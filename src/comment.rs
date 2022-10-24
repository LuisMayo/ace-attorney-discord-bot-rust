use serenity::model::prelude::MessageUpdateEvent;
use Regex;

struct User {
    id: u64,
    name: String,
}

struct Message {
    user: User,
    text: String,
    evidence_path: String
}

impl Message {
    fn new(update: &MessageUpdateEvent) -> Self {
        let author = update.author.unwrap_or_default();
        // let a = update.attachments[0];
        let mut text = update.content.unwrap_or_default();
        Regex::new(r"(https?)\S*")
    }
}