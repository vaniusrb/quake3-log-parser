use crate::entities::{game_match::GameMatch, means_of_death::MeansOfDeath, player::Player};
use std::mem;

/// Store matches list. It's not aware of log events or parser routines.
#[derive(Debug, Clone, Default)]
pub struct MatchesAccumulator {
    current_match: GameMatch,
    matches: Vec<GameMatch>,
    errors: Vec<String>,
}

impl MatchesAccumulator {
    /// Initiate a new match
    pub fn new_match(&mut self) {
        let last_match = mem::take(&mut self.current_match);
        self.matches.push(last_match);
    }

    /// Add player logged.
    pub fn add_player(&mut self, player: impl Into<Player>) {
        self.current_match.players.push(player.into());
    }

    /// Add killed by world.
    pub fn add_kill(&mut self, killer: impl Into<Player>, means_of_death: MeansOfDeath) {
        self.current_match.total_kills += 1;
        self.current_match
            .kills
            .entry(killer.into())
            .and_modify(|c| *c += 1)
            .or_insert(1);
        self.add_means_of_death(means_of_death);
    }

    /// Add killed by world.
    pub fn killed_by_world(&mut self, killed: impl Into<Player>, means_of_death: MeansOfDeath) {
        self.current_match.total_kills += 1;
        self.current_match
            .kills
            .entry(killed.into())
            .and_modify(|c| {
                *c = c.checked_sub(1).unwrap_or_default();
            });
        self.add_means_of_death(means_of_death);
    }

    /// Add means of death.
    fn add_means_of_death(&mut self, means_of_death: MeansOfDeath) {
        self.current_match
            .means_of_death
            .entry(means_of_death)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    /// Register an error.
    pub fn add_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }

    /// Returns the number of matches.
    pub fn len(&self) -> usize {
        self.matches.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns all matches collected.
    pub fn all_matches(mut self) -> Vec<GameMatch> {
        self.matches.push(self.current_match);
        self.matches
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        accumulator::matches_accumulator::MatchesAccumulator,
        entities::means_of_death::MeansOfDeath,
    };

    #[test]
    fn match_analyzer_new_match_test() {
        let mut matches = MatchesAccumulator::default();
        assert_eq!(0, matches.len());
        matches.new_match();
        assert_eq!(1, matches.len());
    }

    #[test]
    fn match_analyzer_add_player_test() {
        let mut matches = MatchesAccumulator::default();
        matches.add_player("Stallone");
        assert!(matches
            .all_matches()
            .iter()
            .any(|m| m.players.iter().any(|p| *p == "Stallone".into())));
    }

    #[test]
    fn match_analyzer_kill_test() {
        let mut matches = MatchesAccumulator::default();
        matches.add_kill("Stallone", MeansOfDeath::ModBfg);
        assert!(matches.clone().all_matches().iter().any(|m| m
            .kills
            .iter()
            .any(|p| *p.0 == "Stallone".into() && *p.1 == 1)));

        assert!(matches.all_matches().iter().any(|m| m
            .means_of_death
            .iter()
            .any(|p| *p.0 == MeansOfDeath::ModBfg)));
    }

    #[test]
    fn match_analyzer_total_kill_test() {
        let mut matches = MatchesAccumulator::default();
        matches.add_kill("Stallone", MeansOfDeath::ModBfg);
        matches.add_kill("Stallone", MeansOfDeath::ModBfg);
        matches.add_kill("Rambo", MeansOfDeath::ModBfgSplash);
        assert_eq!(3, matches.all_matches().first().unwrap().total_kills);
    }

    #[test]
    fn match_analyzer_kill_by_world_test() {
        let mut matches = MatchesAccumulator::default();
        matches.add_kill("Stallone", MeansOfDeath::ModBfg);
        assert!(matches.clone().all_matches().iter().any(|m| m
            .kills
            .iter()
            .any(|p| *p.0 == "Stallone".into() && *p.1 == 1)));

        matches.killed_by_world("Stallone", MeansOfDeath::ModFalling);
        assert!(matches.clone().all_matches().iter().any(|m| m
            .kills
            .iter()
            .any(|p| *p.0 == "Stallone".into() && *p.1 == 0)));
    }
}
