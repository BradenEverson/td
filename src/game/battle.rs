use uuid::Uuid;

pub struct Battle {
    pub team_a: Team,
    pub team_b: Team,
}

impl Battle {
    pub fn start_battle(user_a: Uuid, user_b: Uuid) -> Self {
        Self {
            team_a: Team::new_team(user_a),
            team_b: Team::new_team(user_b),
        }
    }
}

pub struct Team {
    pub player: Uuid,
    pub tower: Tower,
}

impl Team {
    pub fn new_team(user: Uuid) -> Self {
        Self {
            player: user,
            tower: Tower::default(),
        }
    }
}

pub struct Tower {
    pub health: usize,
}

impl Default for Tower {
    fn default() -> Self {
        Self { health: 1500 }
    }
}
