use cards::CARDS;

use super::entity::Unit;

mod cards;

#[allow(non_snake_case)]
pub fn UNITS<'a>() -> Vec<Unit<'a>> {
    CARDS
        .iter()
        .map(|card_str| serde_json::from_str::<Unit>(card_str).unwrap())
        .collect()
}
