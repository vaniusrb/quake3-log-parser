use super::Report;
use crate::{accumulator::match_ranking::MatchRanking, entities::means_of_death::MeansOfDeath};
use serde_json::{json, Value};

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
        let gkms = matches_to_kills_by_means(matches);
        serde_json::to_string_pretty(&gkms).unwrap()
    }
}

// "game-1": {
//     "kills_by_means": {
//       "MOD_SHOTGUN": 10,
//       "MOD_RAILGUN": 2,
//       "MOD_GAUNTLET": 1,
//       ...
//     }
//   }

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

fn matches_to_kills_by_means(matches_r: Vec<MatchRanking>) -> Value {
    let mut j = json!({});
    let map = j.as_object_mut().unwrap();
    for mr in matches_r {
        let kms = kills_by_means_sorted_json(mr.means);
        map.insert(format!("game-{}", mr.id), kms);
    }
    j
}

fn kills_by_means_sorted_json(kills_by_means: Vec<(MeansOfDeath, u32)>) -> Value {
    let mut j = json!({});
    let map = j.as_object_mut().unwrap();
    for km in kills_by_means {
        map.insert(km.0.to_string(), km.1.into());
    }
    j
}

//  */
#[cfg(test)]
mod tests {
    use crate::entities::means_of_death::MeansOfDeath;
    use serde_json::json;

    #[test]
    fn test() {
        let mut j = json!({});
        let map = j.as_object_mut().unwrap();
        map.insert("game-7".into(), 1.into());
        map.insert("game-1".into(), 5.into());
        map.insert("game-3".into(), 9.into());

        let s = serde_json::to_string_pretty(&j).unwrap();
        println!("{s}");
    }
}
