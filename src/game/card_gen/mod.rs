use std::sync::LazyLock;

use cards::CARDS;

use super::entity::Unit;

mod cards;

pub const UNITS: LazyLock<Vec<Unit<'static>>> = LazyLock::new(|| {
    CARDS
        .iter()
        .map(|card_str| serde_json::from_str::<Unit>(card_str).unwrap())
        .collect()
});
