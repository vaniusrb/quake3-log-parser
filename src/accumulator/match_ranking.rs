use crate::entities::{game_match::GameMatch, means_of_death::MeansOfDeath, player::Player};

/// Ranking of a game match.
pub struct MatchRanking {
    pub id: u32,
    pub total_kills: u32,
    pub players: Vec<Player>,
    pub ranking: Vec<(Player, u32)>,
    pub means: Vec<(MeansOfDeath, u32)>,
}

impl MatchRanking {
    pub fn new(g_match: GameMatch) -> Self {
        let mut ranking: Vec<(Player, u32)> = g_match
            .kills
            .into_iter()
            .map(|m| (m.0, m.1))
            .collect::<Vec<_>>();
        ranking.sort_by(|a, b| b.1.cmp(&a.1));
        let mut means: Vec<(MeansOfDeath, u32)> = g_match
            .means_of_death
            .into_iter()
            .map(|m| (m.0, m.1))
            .collect::<Vec<_>>();
        means.sort_by(|a, b| b.1.cmp(&a.1));
        Self {
            id: g_match.id,
            total_kills: g_match.total_kills,
            players: g_match.players,
            ranking,
            means,
        }
    }
}

#[cfg(test)]
mod testes {
    use crate::{
        accumulator::{match_ranking::MatchRanking, matches_accumulator::MatchesAccumulator},
        entities::{means_of_death::MeansOfDeath, player::Player},
    };

    #[test]
    fn match_ranking_test() {
        let mut matches = MatchesAccumulator::default();
        matches.add_kill("Terminator", MeansOfDeath::ModBfg);
        matches.add_kill("Terminator", MeansOfDeath::ModBfg);
        matches.add_kill("Rambo", MeansOfDeath::ModBfgSplash);
        let m = matches.all_matches().remove(0);
        let mr = MatchRanking::new(m);
        assert_eq!(3, mr.total_kills);
        assert_eq!(Player("Terminator".into()), mr.ranking[0].0);
        assert_eq!(2, mr.ranking[0].1);
        assert_eq!(Player("Rambo".into()), mr.ranking[1].0);
        assert_eq!(1, mr.ranking[1].1);
    }
}
