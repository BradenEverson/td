use uuid::Uuid;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Battle {
    pub team_a: Uuid,
    pub team_b: Uuid,
}

impl Battle {
    pub fn start_battle(user_a: Uuid, user_b: Uuid) -> Self {
        Self {
            team_a: user_a,
            team_b: user_b,
        }
    }

    pub fn get_enemy(&self, id: Uuid) -> Uuid {
        if self.team_a == id {
            self.team_b
        } else {
            self.team_a
        }
    }
}
