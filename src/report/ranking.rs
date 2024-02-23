use crate::{accumulator::match_ranking::MatchRanking, report::Report};

/// Players ranking report.
pub struct RankingReport {}

impl RankingReport {
    /// Creates a new ´RankingReport`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RankingReport {
    /// Creates a new ´RankingReport`.
    fn default() -> Self {
        Self::new()
    }
}

impl Report for RankingReport {
    fn report(&self, matches: Vec<MatchRanking>) -> String {
        let mut result = String::new();
        for s_match in matches {
            result.push_str(&report_match(s_match))
        }
        result
    }
}

/// Generates string report from a `MatchRanking`.
fn report_match(s_match: MatchRanking) -> String {
    let mut lines = format!(
        "Match {}\nTotal kills: {}\nRanking:\n",
        s_match.id, s_match.total_kills,
    );
    for (i, (player, kills)) in s_match.ranking.into_iter().enumerate() {
        lines.push_str(&format!("{} - {player}: {kills}\n", i + 1));
    }
    lines.push('\n');
    lines
}
