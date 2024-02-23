use super::Report;
use crate::{accumulator::match_ranking::MatchRanking, entities::means_of_death::MeansOfDeath};
use serde_json::{json, Value};

/// Kills by means ranking report.
pub struct KillsByMeansReport {}

impl KillsByMeansReport {
    /// Creates a new `KillsByMeansReport`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for KillsByMeansReport {
    /// Creates a new `KillsByMeansReport`.
    fn default() -> Self {
        Self::new()
    }
}

impl Report for KillsByMeansReport {
    fn report(&self, matches: Vec<MatchRanking>) -> String {
        let gkms = matches_to_kills_by_means(matches);
        serde_json::to_string_pretty(&gkms).unwrap()
    }
}

/// Return json object respecting the insert order by game matches.
fn matches_to_kills_by_means(matches_r: Vec<MatchRanking>) -> Value {
    let mut j = json!({});
    let map = j.as_object_mut().unwrap();
    for mr in matches_r {
        let kms = kills_by_means_sorted_json(mr.means);
        map.insert(format!("game-{}", mr.id), kms);
    }
    j
}

/// Return json object respecting the insert order by means of death ranking.
fn kills_by_means_sorted_json(kills_by_means: Vec<(MeansOfDeath, u32)>) -> Value {
    let mut j = json!({});
    let map = j.as_object_mut().unwrap();
    for km in kills_by_means {
        println!("{}: {}", km.0, km.1);
        map.insert(km.0.to_string(), km.1.into());
    }
    j
}
