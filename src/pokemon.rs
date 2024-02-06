use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Template)]
#[template(path = "pokemon.html")]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    pub implemented: Option<bool>,
    pub name: String,
    pub national_pokedex_number: u32,
    pub primary_type: String,
    pub secondary_type: Option<String>,
    pub abilities: Vec<String>,
    pub base_stats: BaseStats,
    pub catch_rate: u32,
    pub male_ratio: f32,
    pub shoulder_mountable: Option<bool>,
    pub forms: Option<Vec<Form>>,
    pub behaviour: Option<Behaviour>,
    pub base_experience_yield: u32,
    pub experience_group: String,
    pub egg_cycles: u32,
    pub egg_groups: Vec<String>,
    pub drops: Option<Drops>,
    pub moves: Vec<String>,
    pub labels: Vec<String>,
    pub pokedex: Option<Vec<String>>,
    pub pre_evolution: Option<String>,
    pub evolutions: Vec<Evolution>,
    pub base_scale: Option<f32>,
    pub hitbox: Option<Hitbox>,
    pub base_friendship: u32,
    pub ev_yield: EvYield,
    pub height: u32,
    pub weight: u32,
    pub aspects: Vec<String>,
    pub cannot_dynamax: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BaseStats {
    pub hp: u32,
    pub attack: u32,
    pub defence: u32,
    pub special_attack: u32,
    pub special_defence: u32,
    pub speed: u32,
}

// -- Behaviour

#[derive(Deserialize, Serialize, Debug)]
pub struct Behaviour {
    pub moving: Option<Moving>,
    pub resting: Option<Resting>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resting {
    pub can_sleep: bool,
    pub will_sleep_on_bed: Option<bool>,
    pub depth: Option<String>,
    pub light: Option<Light>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Moving {
    pub fly: Option<Fly>,
    pub walk: Option<Walk>,
    pub swim: Option<Swim>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fly {
    pub can_fly: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Walk {
    pub can_walk: Option<bool>,
    pub avoids_land: Option<bool>,
    pub walk_speed: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Swim {
    pub swim_speed: Option<f32>,
    pub can_swim_in_water: Option<bool>,
    pub can_breathe_underwater: Option<bool>,
    pub can_walk_on_water: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Light {
    Int(i32),
    Range(String),
}

// -- !Behaviour

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub name: String,
    pub primary_type: Option<String>,
    pub secondary_type: Option<String>,
    pub abilities: Option<Vec<String>>,
    pub base_stats: Option<BaseStats>,
    pub catch_rate: Option<u32>,
    pub male_ratio: Option<f32>,
    pub base_experience_yield: Option<u32>,
    pub base_friendship: Option<u32>,
    pub ev_yield: Option<EvYield>,
    pub experience_group: Option<String>,
    pub egg_cycles: Option<u32>,
    pub egg_groups: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub aspects: Option<Vec<String>>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub evolutions: Option<Vec<Evolution>>,
    pub cannot_dynamax: Option<bool>,
    pub battle_only: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EvYield {
    pub hp: u32,
    pub attack: u32,
    pub defence: u32,
    pub special_attack: u32,
    pub special_defence: u32,
    pub speed: u32,
}

// -- Drops
#[derive(Deserialize, Serialize, Debug)]
pub struct Drops {
    pub amount: u32,
    pub entries: Vec<Entry>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub item: String,
    pub quantity_range: Option<String>,
    pub percentage: Option<f32>,
}

// -- !Drops

#[derive(Deserialize, Serialize, Debug)]
pub struct Hitbox {
    pub width: f32,
    pub height: f32,
    pub fixed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Evolution {
    pub id: String,
    pub variant: String,
    pub result: String,
    pub consume_held_item: bool,
    pub learnable_moves: Vec<String>,
    pub requirements: Vec<Requirement>,
    pub required_context: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Requirement {
    variant: String,
    min_level: Option<u32>,
    target: Option<String>,
}
