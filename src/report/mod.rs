use crate::accumulator::match_ranking::MatchRanking;

pub mod ranking;
pub mod kills_by_means;

/// Parser trait, used to extract log event from a string row.
pub trait Report {
    fn report(&self, matches: Vec<MatchRanking>) -> String;
}
