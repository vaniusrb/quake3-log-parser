use crate::means_of_death::MeansOfDeath;
use ahash::{HashMap, HashMapExt};
use std::sync::Mutex;

static MATCH_COUNTER: Mutex<u32> = Mutex::new(0u32);

#[derive(Debug)]
pub struct SingleMatch {
    pub id: u32,
    pub total_kills: u32,
    pub players: Vec<String>,
    pub kills: HashMap<String, u32>,
    pub means_of_death: HashMap<MeansOfDeath, u32>,
}

impl Default for SingleMatch {
    fn default() -> Self {
        // Get current match id
        let mut guard = MATCH_COUNTER.lock().unwrap();
        let id: u32 = *guard;
        // Increment arena id
        *guard += 1;
        Self {
            id,
            kills: HashMap::<String, u32>::with_capacity(1000),
            means_of_death: HashMap::<MeansOfDeath, u32>::with_capacity(1000),
            total_kills: 0,
            players: vec![],
        }
    }
}
