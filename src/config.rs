use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub discord_token: String,
    pub invite_link: String,
}

impl Config {
    pub fn load_env() ->Result<Self, env::VarError> {
        Ok(Self {
            discord_token: env::var("DISCORD_TOKEN")?,
            invite_link: env::var("INVITE_LINK")?,
        })
    }
}