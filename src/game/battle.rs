use crate::server::user::User;

pub struct Battle<'a> {
    pub team_a: Team<'a>,
    pub team_b: Team<'a>
}

impl<'a> Battle<'a> {
    pub fn start_battle(user_a: &'a User, user_b: &'a User) -> Self {
        Self { 
            team_a: Team::new_team(user_a),
            team_b: Team::new_team(user_b) 
        }
    }
}

pub struct Team<'a> {
    pub player: &'a User,
    pub tower: Tower
}

impl<'a> Team<'a> {
    pub fn new_team(user: &'a User) -> Self {
        Self { player: user, tower: Tower::default() }
    }
}

pub struct Tower {
    pub health: usize
}

impl Default for Tower {
    fn default() -> Self {
        Self { health: 1500 }
    }
}
