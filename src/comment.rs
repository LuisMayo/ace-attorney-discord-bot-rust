use pyo3::{Py, Python, types::{PyModule, IntoPyDict}, PyAny};
use serenity::model::channel::Message;
use regex::Regex;
struct User {
    id: u64,
    name: String,
}

pub struct Comment {
    user: User,
    text: String,
    evidence_path: Option<String>
}

impl Comment {
    pub fn new(update: &Message) -> Self {
        let url_regex: Regex = Regex::new(r"(https?)\S*").unwrap();
        let custom_emoji_regex: Regex = Regex::new(r"<[a]?:\w{2,32}:\d{18}>").unwrap();
        let emoji_regex: Regex = Regex::new(r":\w{2,32}:").unwrap();
        // const URL_REGEX: Regex = Regex::new(r"(https?)\S*").unwrap();
        let author = &update.author;
        // let a = update.attachments[0];
        let original_text = &update.content;
        let mut text = url_regex.replace_all(original_text, "(link)").to_string();
        text = custom_emoji_regex.replace_all(&text, "").to_string();
        text = emoji_regex.replace_all(&text, "").to_string();
        return Self {
            user: User {
                id: author.id.0,
                name: author.name.clone()
            },
            text,
            evidence_path: None
        };
    }

    pub fn to_comment <'a, 'b> (&'a self, py: &'b Python, engine: &'b PyModule) -> &'b PyAny {
        let kwargs = [("text_content", &self.text),("user_name", &self.user.name)].into_py_dict(*py);
        return engine.getattr("comment").unwrap().call_method("Comment", ("", ), Some(kwargs)).unwrap();
    } 
}