// struct Game

use super::Report;
use crate::{accumulator::match_ranking::MatchRanking, entities::means_of_death::MeansOfDeath};
use ahash::HashMap;
use serde::Serialize;

#[derive(Serialize)]
struct KillsByMeans {
    kills_by_means: HashMap<MeansOfDeath, u32>,
}

pub struct KillsByMeansReport {}

impl KillsByMeansReport {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for KillsByMeansReport {
    fn default() -> Self {
        Self::new()
    }
}

impl Report for KillsByMeansReport {
    fn report(&self, matches: Vec<MatchRanking>) -> String {
        todo!()
    }
}

// /**
//  *
//  "game_1": {
// "total_kills": 45,
// "players": ["Dono da bola", "Isgalamido", "Zeh"],
// "kills": {
//   "Dono da bola": 5,
//   "Isgalamido": 18,
//   "Zeh": 20
//   }
// }*

//  */
