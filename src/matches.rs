use crate::{means_of_death::MeansOfDeath, single_match::SingleMatch};
use std::mem;

#[derive(Debug, Default)]
pub struct Matches {
    current_match: SingleMatch,
    matches: Vec<SingleMatch>,
    errors: Vec<String>,
}

impl Matches {
    pub fn new_match(&mut self) {
        let last_match = mem::take(&mut self.current_match);
        self.matches.push(last_match);
    }

    pub fn add_player(&mut self, player: &str) {
        self.current_match.players.push(player.to_string());
    }

    pub fn add_kill(&mut self, killer: &str, means_of_death: MeansOfDeath) {
        self.current_match.total_kills += 1;
        self.current_match
            .kills
            .entry(killer.into())
            .and_modify(|c| *c += 1)
            .or_insert(1);
        self.add_means_of_death(means_of_death);
    }

    pub fn killed_by_world(&mut self, killed: &str, means_of_death: MeansOfDeath) {
        self.current_match.total_kills += 1;
        self.current_match
            .kills
            .entry(killed.into())
            .and_modify(|c| {
                *c = c.checked_sub(1).unwrap_or_default();
            });
        self.add_means_of_death(means_of_death);
    }

    fn add_means_of_death(&mut self, means_of_death: MeansOfDeath) {
        self.current_match
            .means_of_death
            .entry(means_of_death)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    pub fn add_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }

    pub fn all_matches(mut self) -> Vec<SingleMatch> {
        self.matches.push(self.current_match);
        self.matches
    }
}
