use crate::entities::log_event::LogEvent;
use crate::entities::means_of_death::MeansOfDeath;
use crate::parser::Parser;
use once_cell::sync::Lazy;
use regex::Regex;

/// World "player", used when the player is dead by the world environment.
const WORLD: &str = "<world>";
/// Regex to detect when a new game is initiated.
static INIT_GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#".*\d:\d\d InitGame:"#).unwrap());
/// Regex to intercept when a player is connected.
static PLAYER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#".*\d:\d\d ClientUserinfoChanged: \d+ n\\(?P<player>.*?)\\.*"#).unwrap()
});
/// Regex to intercept when a kill happens.
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
    /// Creates a new `RegexParser`.
    pub fn new() -> Self {
        Self {
            init_game_regex: &INIT_GAME_REGEX,
            player_regex: &PLAYER_REGEX,
            kill_regex: &KILL_REGEX,
        }
    }
}

impl Default for RegexParser {
    /// Creates a new `RegexParser`.
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

#[cfg(test)]
mod tests {
    use super::RegexParser;
    use crate::{
        entities::{log_event::LogEvent, means_of_death::MeansOfDeath},
        parser::Parser,
    };

    #[test]
    fn init_game_test() {
        let parser = RegexParser::new();
        let row = r#"20:37 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\
0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\bot_minplayers\
0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\
protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0"#;
        assert_eq!(LogEvent::NewMatch, parser.parse(row).unwrap());
    }

    #[test]
    fn init_add_player_test() {
        let parser = RegexParser::new();
        let row = r#"20:38 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\uriel/zael\hmodel\uriel/zael\
g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0"#;
        assert_eq!(
            LogEvent::AddPlayer("Isgalamido".into()),
            parser.parse(row).unwrap()
        );
    }

    #[test]
    fn add_kill_test() {
        let parser = RegexParser::new();
        let row = r#"20:54 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT"#;
        assert_eq!(
            LogEvent::KilledByWorld {
                killed: "Isgalamido".into(),
                means: MeansOfDeath::ModTriggerHurt
            },
            parser.parse(row).unwrap()
        );
    }

    #[test]
    fn add_kill_by_world_test() {
        let parser = RegexParser::new();
        let row = r#"1:08 Kill: 3 2 6: Isgalamido killed Mocinha by MOD_ROCKET"#;
        assert_eq!(
            LogEvent::Kill {
                killer: "Isgalamido".into(),
                means: MeansOfDeath::ModRocket
            },
            parser.parse(row).unwrap()
        );
    }

    #[test]
    fn error_test() {
        let parser = RegexParser::new();
        let row = r#"1:08 Kill: 3 2 6: Isgalamido killed Mocinha by MOD_INVALID"#;
        assert!(parser.parse(row).is_err());
    }
}
