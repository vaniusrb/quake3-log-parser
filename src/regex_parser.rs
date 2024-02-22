use crate::log_event::LogEvent;
use crate::parser::Parser;
use crate::MeansOfDeath;
use once_cell::sync::Lazy;
use regex::Regex;

/// World "player", used when the player is dead by the world environment.
const WORLD: &str = "<world>";
/// Regex to detect when a new game is initiated.
static INIT_GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#".*\d:\d\d InitGame:"#).unwrap());
/// Regex to detected when a player is connected.
static PLAYER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#".*\d:\d\d ClientUserinfoChanged: \d+ n\\(?P<player>.*?)\\.*"#).unwrap()
});
/// Regex to detected when a kill happens.
static KILL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#".*\d:\d\d (Kill: \d* \d* \d*): (?P<killer>.*) killed (?P<killed>.*) by (?P<means>.*)"#,
    )
    .unwrap()
});

/// Parser log file using static regex.
pub struct RegexParser {
    init_game_regex: &'static Regex,
    player_regex: &'static Regex,
    kill_regex: &'static Regex,
}

impl RegexParser {
    pub fn new() -> Self {
        Self {
            init_game_regex: &INIT_GAME_REGEX,
            player_regex: &PLAYER_REGEX,
            kill_regex: &KILL_REGEX,
        }
    }
}

impl Default for RegexParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for RegexParser {
    /// Extract event from log row.
    fn parse(&self, row: &str) -> Result<LogEvent, String> {
        // New match
        if self.init_game_regex.is_match(row) {
            return Ok(LogEvent::NewMatch);
        }
        // Logged player
        if let Some(captures) = self.player_regex.captures(row) {
            if let Some(player) = captures.name("player") {
                return Ok(LogEvent::AddPlayer(player.as_str().into()));
            }
        }
        // Kill player
        if let Some(captures) = self.kill_regex.captures(row) {
            if let (Some(killer), Some(killed), Some(means)) = (
                captures.name("killer"),
                captures.name("killed"),
                captures.name("means"),
            ) {
                match MeansOfDeath::try_from(means.as_str()) {
                    Ok(means) => {
                        let kill = if killer.as_str() == WORLD {
                            LogEvent::KilledByWorld {
                                killed: killed.as_str().into(),
                                means,
                            }
                        } else {
                            LogEvent::Kill {
                                killer: killer.as_str().into(),
                                means,
                            }
                        };
                        return Ok(kill);
                    }
                    Err(e) => {
                        return Err(format!(
                            "error to parse MeansOfDeath `{}`: {e}",
                            means.as_str()
                        ))
                    }
                }
            }
        }
        Ok(LogEvent::Other)
    }
}
