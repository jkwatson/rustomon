use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawMonster {
    pub name: String,
    pub tags: String,
    pub level: String,
    pub biome: String,
    pub alignment: String,
    #[serde(rename = "move")]
    pub move_amount: String,
    pub attack: String,
    pub page: String,
    pub statblock: String,
    pub source: String,
}

#[derive(Debug)]
pub struct Monster {
    pub name: String,
    pub tags: Vec<String>,
    pub level: i32,
    pub biomes: Vec<String>,
    pub alignment: String,
    pub move_amount: String,
    pub attack: String,
    pub page: String,
    pub statblock: String,
    pub source: String,
}
