use crate::entities::log_event::LogEvent;

pub mod regex_parser;

/// Parser trait, used to extract log event from a string row.
pub trait Parser {
    fn parse(&self, row: &str) -> Result<LogEvent, String>;
}
