use crate::{log_event::LogEvent, matches::Matches, parser::Parser};

pub struct Analyzer<T: Parser> {
    matches: Matches,
    parser: T,
}

impl<T: Parser> Analyzer<T> {
    pub fn new(parser: T, matches: Matches) -> Self {
        Self { matches, parser }
    }

    /// Analyze event from log row, updating info to current match if necessary.
    pub fn digest(mut self, row: &&str) -> Self {
        match self.parser.parse(row) {
            Ok(event) => match event {
                LogEvent::NewMatch => self.matches.new_match(),
                LogEvent::AddPlayer(player) => self.matches.add_player(&player),
                LogEvent::Kill { killer, means } => {
                    self.matches.add_kill(&killer, means);
                }
                LogEvent::KilledByWorld { killed, means } => {
                    self.matches.killed_by_world(&killed, means)
                }
                LogEvent::Other => {}
            },
            Err(e) => self.matches.add_error(&e),
        }
        self
    }

    pub fn matches(self) -> Matches {
        self.matches
    }
}
