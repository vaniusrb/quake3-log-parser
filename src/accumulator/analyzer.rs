use super::matches_accumulator::MatchesAccumulator;
use crate::{entities::log_event::LogEvent, parser::Parser};

/// `MatchAnalyzer` is responsible to analyze a log event and
pub struct MatchAnalyzer<T: Parser> {
    matches: MatchesAccumulator,
    parser: T,
}

impl<T: Parser> MatchAnalyzer<T> {
    pub fn new(parser: T, matches: MatchesAccumulator) -> Self {
        Self { matches, parser }
    }

    /// Analyze event from log row, updating info to current match if necessary.
    pub fn digest(mut self, row: &str) -> Self {
        match self.parser.parse(row) {
            Ok(event) => match event {
                LogEvent::NewMatch => self.matches.new_match(),
                LogEvent::AddPlayer(player) => self.matches.add_player(player),
                LogEvent::Kill { killer, means } => {
                    self.matches.add_kill(killer, means);
                }
                LogEvent::KilledByWorld { killed, means } => {
                    self.matches.killed_by_world(killed, means)
                }
                LogEvent::Other => {}
            },
            Err(e) => self.matches.add_error(&e),
        }
        self
    }

    pub fn matches(self) -> MatchesAccumulator {
        self.matches
    }
}

#[cfg(test)]
mod tests {
    use super::MatchAnalyzer;
    use crate::{
        accumulator::matches_accumulator::MatchesAccumulator,
        entities::{log_event::LogEvent, means_of_death::MeansOfDeath},
        parser::Parser,
    };

    struct MockParser {
        event: LogEvent,
    }

    impl MockParser {
        fn new(event: LogEvent) -> Self {
            Self { event }
        }
    }

    impl Parser for MockParser {
        fn parse(&self, _row: &str) -> Result<LogEvent, String> {
            Ok(self.event.clone())
        }
    }

    #[test]
    fn match_analyzer_new_match_test() {
        let parser = MockParser::new(LogEvent::NewMatch);
        let matches = MatchesAccumulator::default();
        assert_eq!(0, matches.len());
        let matches = MatchAnalyzer::new(parser, matches).digest("").matches();
        assert_eq!(1, matches.len());
    }

    #[test]
    fn match_analyzer_add_player_test() {
        let parser = MockParser::new(LogEvent::AddPlayer("Stallone".into()));
        let matches = MatchesAccumulator::default();
        let matches = MatchAnalyzer::new(parser, matches).digest("").matches();
        assert!(matches
            .all_matches()
            .iter()
            .any(|m| m.players.iter().any(|p| *p == "Stallone".into())));
    }

    #[test]
    fn match_analyzer_kill_test() {
        let parser = MockParser::new(LogEvent::Kill {
            killer: "Stallone".into(),
            means: MeansOfDeath::ModBfg,
        });
        let matches = MatchesAccumulator::default();
        let matches = MatchAnalyzer::new(parser, matches).digest("").matches();
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
    fn match_analyzer_kill_by_world_test() {
        let mut matches = MatchesAccumulator::default();
        let parser = MockParser::new(LogEvent::Kill {
            killer: "Stallone".into(),
            means: MeansOfDeath::ModBfg,
        });
        matches = MatchAnalyzer::new(parser, matches).digest("").matches();
        assert!(matches.clone().all_matches().iter().any(|m| m
            .kills
            .iter()
            .any(|p| *p.0 == "Stallone".into() && *p.1 == 1)));

        let parser = MockParser::new(LogEvent::KilledByWorld {
            killed: "Stallone".into(),
            means: MeansOfDeath::ModFalling,
        });
        matches = MatchAnalyzer::new(parser, matches).digest("").matches();
        assert!(matches.clone().all_matches().iter().any(|m| m
            .kills
            .iter()
            .any(|p| *p.0 == "Stallone".into() && *p.1 == 0)));
    }
}
