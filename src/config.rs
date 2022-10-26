use std::collections::HashMap;

use config::Config;



pub struct Settings {
    pub token: String,
    pub prefix: char,
    pub invite_link: String
}

impl serenity::prelude::TypeMapKey for Settings {
     type Value = Settings;
}

impl Settings {
    pub fn new() -> Self {
        let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("./config.yaml"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("DISCORD"))
        .build()
        .unwrap_or_default();
        // foo.
        // Print out our settings (as a HashMap)
        let settingsMap = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
        return Self {
            token: settingsMap.get("token").unwrap().to_owned(),
            prefix: settingsMap.get("prefix").unwrap_or(&String::from("!")).as_str().chars().next().unwrap_or('!'),
            invite_link: settingsMap.get("invite_link").unwrap_or(&String::from("Invite link not in config file")).to_owned()
        }
    }
}