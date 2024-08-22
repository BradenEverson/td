use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use super::card_gen::UNITS;

#[derive(Default, PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
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

impl<'a> Unit<'a> {
    pub fn get_name(&self) -> &str {
        self.name
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum AttackType {
    Area,
    #[default]
    Single,
}

pub fn draw_hand<'a, const NUM: usize>() -> Option<[Unit<'a>; NUM]> {
    let cards = UNITS();
    if NUM > cards.len() {
        return None;
    }

    let mut units = [Unit::default(); NUM];
    let mut cards_available = cards;

    let mut rng = rand::thread_rng();

    cards_available.shuffle(&mut rng);

    for unit in units.iter_mut().take(NUM) {
        if let Some(drawn) = cards_available.pop() {
            *unit = drawn;
        } else {
            return None;
        }
    }

    Some(units)
}

#[cfg(test)]
mod tests {
    use super::{draw_hand, Unit};
    const LARGE_NUMBER: usize = 9999;

    #[test]
    fn draw_hand_greater_than_cards_len_is_none() {
        let attempted_draw = draw_hand::<LARGE_NUMBER>();

        assert!(attempted_draw.is_none())
    }

    #[test]
    fn draw_hand_less_than_cards_len_is_some_and_valid() {
        let valid_draw = draw_hand::<2>();

        assert!(valid_draw.is_some_and(|hand| hand.len() == 2))
    }

    #[test]
    fn draw_hand_actually_draws_uniquely_and_not_default_units() {
        let valid_draw: [Unit; 5] = draw_hand().expect("Draw a hand of 5 units");
        for unit in valid_draw {
            assert_ne!(unit, Unit::default())
        }
    }
}
