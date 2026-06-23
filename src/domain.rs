use std::cmp::{Ord, Ordering};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        let id = Uuid::new_v4();
        Self { id, name }
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Match(Player, Player);

impl Match {
    pub fn new(p1: &Player, p2: &Player) -> Self {
        Self(p1.clone(), p2.clone())
    }

    pub fn player1(&self) -> &Player {
        &self.0
    }

    pub fn player2(&self) -> &Player {
        &self.1
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Round(Vec<Match>);

impl Round {
    pub fn new(matches: &[Match]) -> Self {
        Self(Vec::from(matches))
    }

    pub fn matches(&self) -> impl Iterator<Item = &Match> {
        self.0.iter()
    }
}
