use crate::single_match::MatchAccumulator;

pub struct MatchRanking {
    pub id: u32,
    pub total_kills: u32,
    pub players: Vec<String>,
    pub ranking: Vec<(String, u32)>,
    pub means: Vec<(crate::means_of_death::MeansOfDeath, u32)>,
}

impl MatchRanking {
    pub fn new(s_match: MatchAccumulator) -> Self {
        let mut ranking: Vec<(String, u32)> = s_match
            .kills
            .into_iter()
            .map(|m| (m.0, m.1))
            .collect::<Vec<_>>();
        ranking.sort_by(|a, b| b.1.cmp(&a.1));
        let mut means: Vec<(crate::means_of_death::MeansOfDeath, u32)> = s_match
            .means_of_death
            .into_iter()
            .map(|m| (m.0, m.1))
            .collect::<Vec<_>>();
        means.sort_by(|a, b| b.1.cmp(&a.1));
        Self {
            id: s_match.id,
            total_kills: s_match.total_kills,
            players: s_match.players,
            ranking,
            means,
        }
    }
}
