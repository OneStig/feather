use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub discord_token: String,
    pub invite_link: String,
    pub mongodb_uri: String,
    pub steamweb_token: String,
}

impl Config {
    pub fn load_env() ->Result<Self, env::VarError> {
        Ok(Self {
            discord_token: env::var("DISCORD_TOKEN")?,
            invite_link: env::var("INVITE_LINK")?,
            mongodb_uri: env::var("MONGODB_URI")?,
            steamweb_token: env::var("STEAMWEB_TOKEN")?,
        })
    }
}