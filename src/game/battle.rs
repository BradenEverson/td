use uuid::Uuid;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Battle {
    pub team_a: (Uuid, Tower),
    pub team_b: (Uuid, Tower),
}

impl Battle {
    pub fn start_battle(user_a: Uuid, user_b: Uuid) -> Self {
        Self {
            team_a: (user_a, Tower::default()),
            team_b: (user_b, Tower::default()),
        }
    }

    pub fn get_enemy(&self, id: Uuid) -> Uuid {
        if self.team_a.0 == id {
            self.team_b.0
        } else {
            self.team_a.0
        }
    }

    pub fn damage_tick(&mut self, attack_on: Uuid, dmg: usize) -> Option<usize> {
        let tower = if self.team_a.0 == attack_on {
            &mut self.team_a.1
        } else {
            &mut self.team_b.1
        };

        if dmg >= tower.health {
            tower.health = 0;
            None
        } else {
            tower.health -= dmg;

            Some(tower.health)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tower {
    pub health: usize,
}

impl Default for Tower {
    fn default() -> Self {
        Self { health: 15000 }
    }
}
