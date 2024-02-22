use crate::log_event::LogEvent;


/// Parser trait, used to extract log event from a string row.
pub trait Parser {
    fn parse(&self, row: &str) -> Result<LogEvent, String>;
}
