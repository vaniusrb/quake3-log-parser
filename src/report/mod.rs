use crate::accumulator::match_ranking::MatchRanking;

pub mod formatted_report;

/// Parser trait, used to extract log event from a string row.
pub trait Report {
    fn report(&self, match_r: MatchRanking) -> String;
}
