use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryPoint {
    pub value: f64,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: i64,
    pub steam_id: i64,
    pub currency: String,
    pub cooldown: i64,
    pub value_history: Vec<InventoryPoint>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleAssignment {
    pub threshold: f64,
    pub role_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guild {
    pub guild_id: i64,
    pub roles: Vec<RoleAssignment>,
}