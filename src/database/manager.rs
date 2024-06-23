use std::{default, env};
use std::sync::Arc;

use mongodb::{Client, Database, Collection};
use mongodb::bson::{doc, to_bson};
use mongodb::options::UpdateOptions;

use tokio::sync::Mutex;

use super::models::{User, Guild};

pub struct DatabaseManager {
    client: Client,
    db: Database,
    users: Collection<User>,
    guilds: Collection<Guild>,
}

impl DatabaseManager {
    pub async fn new() -> mongodb::error::Result<Arc<Mutex<Self>>> {
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI was not found");
        let client = Client::with_uri_str(&uri).await?;
        let db = client.database("botchicken");
        let users = db.collection("users");
        let guilds = db.collection("guilds");

        println!("Database successfully connected");

        Ok(Arc::new(Mutex::new(Self {
            client,
            db,
            users,
            guilds,
        })))
    }

    pub async fn get_user(&self, user_id: &i64) -> mongodb::error::Result<Option<User>> {
        match self.users.find_one(doc! { "user_id": user_id }, None).await? {
            Some(user) => Ok(Some(user)),
            None => {
                let default_user = User {
                    user_id: *user_id,
                    steam_id: 0,
                    currency: "USD".to_string(),
                    cooldown: 0,
                    value_history: vec![],
                };

                self.create_user(default_user.clone()).await?;
                Ok(Some(default_user))
            }
        }
    }

    pub async fn create_user(&self, user: User) -> mongodb::error::Result<()> {
        self.users.insert_one(user, None).await?;
        Ok(())
    }

    pub async fn update_user(&self, user: &User) -> mongodb::error::Result<()> {
        let filter = doc! { "user_id": &user.user_id };
        let update = doc! { "$set": {
            "currency": &user.currency,
            "steam_id": &user.steam_id
        }};
        let options = UpdateOptions::builder().upsert(true).build();
        
        self.users.update_one(filter, update, options).await?;
        Ok(())
    }

    pub async fn get_guild(&self, guild_id: &i64) -> mongodb::error::Result<Option<Guild>> {
        match self.guilds.find_one(doc! { "guild_id": guild_id }, None).await? {
            Some(guild) => Ok(Some(guild)),
            None => {
                let default_guild = Guild {
                    guild_id: *guild_id,
                    roles: vec![],
                };

                self.create_guild(default_guild.clone()).await?;
                Ok(Some(default_guild))
            }
        }
    }

    pub async fn update_guild(&self, guild: &Guild) -> mongodb::error::Result<()> {
        let filter = doc! { "guild_id": &guild.guild_id };

        let roles_bson = to_bson(&guild.roles)?;
        let update = doc! { "$set": {
            "roles": roles_bson,
        }};
        let options = UpdateOptions::builder().upsert(true).build();
        
        self.guilds.update_one(filter, update, options).await?;
        Ok(())
    }

    pub async fn create_guild(&self, guild: Guild) -> mongodb::error::Result<()> {
        self.guilds.insert_one(guild, None).await?;
        Ok(())
    }
}