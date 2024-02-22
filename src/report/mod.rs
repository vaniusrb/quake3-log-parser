use crate::accumulator::match_ranking::MatchRanking;

pub mod formatted_report;
pub mod json_report;

/// Parser trait, used to extract log event from a string row.
pub trait Report {
    fn report(&self, match_r: MatchRanking) -> String;
}
