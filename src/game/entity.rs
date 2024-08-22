use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Unit<'a> {
    name: &'a str,
    emoji: char,

    cost: usize,
    health: usize,
    power: usize,

    size: f32,
    speed: f32,

    attack_type: AttackType,
}

#[derive(Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AttackType {
    Area,
    #[default]
    Single,
}
