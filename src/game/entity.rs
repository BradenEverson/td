#[derive(Default, PartialEq, Eq, Clone)]
pub struct SpawnableEntity {
    name: String,
    cost: usize,
    health: usize,
    power: usize,

    attack_type: AttackType
}

#[derive(Default, PartialEq, Eq, Clone)]
pub enum AttackType {
    Area,
    #[default]
    Single,
}
