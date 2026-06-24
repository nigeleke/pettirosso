use fastrand;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{Match, Player, Round};

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Page {
    #[default]
    PlayerPool,
    Registration,
    Round,
}

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    page: Page,
    pool: Vec<Player>,
    registered: Vec<Player>,
    rounds: Vec<Round>,
    round_number: usize,
}

impl Model {
    pub fn page(&self) -> &Page {
        &self.page
    }

    pub fn set_page(&mut self, page: Page) {
        self.page = page;
    }

    pub fn pool(&self) -> impl Iterator<Item = &Player> {
        self.pool.iter()
    }

    pub fn add_player(&mut self, name: String) {
        self.pool.push(Player::new(name));
        self.pool.sort();
    }

    pub fn remove_player(&mut self, id: Uuid) {
        self.pool.retain(|p| p.id != id);
        self.deregister(id);
    }

    pub fn available(&self) -> impl Iterator<Item = &Player> {
        self.pool.iter().filter(|p| !self.registered.contains(p))
    }

    pub fn register(&mut self, id: Uuid) {
        let player = self.available().find(|p| p.id == id).cloned();
        player.into_iter().for_each(|p| {
            self.registered.push(p);
            self.registered.sort();
            self.generate_round_robin();
        });
    }

    pub fn deregister(&mut self, id: Uuid) {
        self.registered.retain(|p| p.id != id);
        self.generate_round_robin();
    }

    pub fn registered(&self) -> impl Iterator<Item = &Player> {
        self.registered.iter()
    }

    pub fn generate_round_robin(&mut self) {
        let mut n = self.registered.len();
        let mut players = self.registered.clone();

        if n % 2 == 1 {
            players.push(Player::new("-- bye --".into()));
            n += 1;
        }

        fastrand::shuffle(&mut players);

        self.rounds = Vec::new();
        self.round_number = 0;

        if !players.is_empty() {
            let total_rounds = players.len() - 1;

            (0..total_rounds).for_each(|_| {
                let mut round_matches = Vec::new();

                for i in 0..(n / 2) {
                    let p1 = &players[i];
                    let p2 = &players[n - 1 - i];
                    round_matches.push(Match::new(p1, p2));
                }

                self.rounds.push(Round::new(&round_matches));

                if n > 2 {
                    let first = players[0].clone();
                    let last = players[n - 1].clone();
                    let mut rotated = vec![first];
                    rotated.push(last);
                    rotated.extend_from_slice(&players[1..n - 1]);
                    players = rotated;
                }
            });
        }

        fastrand::shuffle(&mut self.rounds);
    }

    pub fn round_number(&self) -> usize {
        self.round_number
    }

    pub fn rounds(&self) -> impl Iterator<Item = &Round> {
        self.rounds.iter()
    }
}
