use crate::match_ranking::MatchRanking;

/// Parser trait, used to extract log event from a string row.
pub trait Report {
    fn report(&self, match_r: MatchRanking) -> String;
}
