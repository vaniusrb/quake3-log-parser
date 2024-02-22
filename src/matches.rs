use crate::{means_of_death::MeansOfDeath, player::Player, single_match::MatchAccumulator};
use std::mem;

/// Store matches list.
#[derive(Debug, Default)]
pub struct MatchesList {
    match_acc: MatchAccumulator,
    matches: Vec<MatchAccumulator>,
    errors: Vec<String>,
}

impl MatchesList {
    /// Initiate a new match
    pub fn new_match(&mut self) {
        let last_match = mem::take(&mut self.match_acc);
        self.matches.push(last_match);
    }

    /// Add player logged.
    pub fn add_player(&mut self, player: Player) {
        self.match_acc.players.push(player);
    }

    /// Add killed by world.
    pub fn add_kill(&mut self, killer: Player, means_of_death: MeansOfDeath) {
        self.match_acc.total_kills += 1;
        self.match_acc
            .kills
            .entry(killer)
            .and_modify(|c| *c += 1)
            .or_insert(1);
        self.add_means_of_death(means_of_death);
    }

    /// Add killed by world.
    pub fn killed_by_world(&mut self, killed: Player, means_of_death: MeansOfDeath) {
        self.match_acc.total_kills += 1;
        self.match_acc.kills.entry(killed).and_modify(|c| {
            *c = c.checked_sub(1).unwrap_or_default();
        });
        self.add_means_of_death(means_of_death);
    }

    /// Add means of death.
    fn add_means_of_death(&mut self, means_of_death: MeansOfDeath) {
        self.match_acc
            .means_of_death
            .entry(means_of_death)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    /// Register an error.
    pub fn add_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }

    /// Returns all matches collected.
    pub fn all_matches(mut self) -> Vec<MatchAccumulator> {
        self.matches.push(self.match_acc);
        self.matches
    }
}
