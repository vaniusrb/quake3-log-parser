use crate::{accumulator::match_ranking::MatchRanking, report::Report};

pub struct FormattedReport {}

impl FormattedReport {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for FormattedReport {
    fn default() -> Self {
        Self::new()
    }
}

impl Report for FormattedReport {
    fn report(&self, s_match: MatchRanking) -> String {
        let mut lines = format!(
            "Match {}\nTotal kills: {}\nRanking:\n",
            s_match.id, s_match.total_kills,
        );

        for (i, (player, kills)) in s_match.ranking.into_iter().enumerate() {
            lines.push_str(&format!("{} - {player}: {kills}\n", i + 1));
        }

        for (i, (means, kills)) in s_match.means.into_iter().enumerate() {
            lines.push_str(&format!("{} - {means}: {kills}\n", i + 1));
        }

        lines.push('\n');
        lines
    }
}
