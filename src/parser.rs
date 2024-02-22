use crate::log_event::LogEvent;

pub trait Parser {
    fn parse(&self, row: &str) -> Result<LogEvent, String>;
}
