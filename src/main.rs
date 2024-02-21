pub mod means_of_death;

use ahash::{HashMap, HashMapExt};
use means_of_death::MeansOfDeath;
use memmap2::Mmap;
use mimalloc::MiMalloc;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{env::args_os, fs::File, mem, path::Path, sync::Mutex};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

static MATCH_COUNTER: Mutex<u32> = Mutex::new(0u32);

#[derive(Debug)]
struct Match {
    id: u32,
    total_kills: u32,
    players: Vec<String>,
    kills: HashMap<String, u32>,
    means_of_death: HashMap<MeansOfDeath, u32>,
}

impl Default for Match {
    fn default() -> Self {
        // Get current match id
        let mut guard = MATCH_COUNTER.lock().unwrap();
        let id: u32 = *guard;
        // Increment arena id
        *guard += 1;
        Self {
            id,
            kills: HashMap::<String, u32>::with_capacity(1000),
            means_of_death: HashMap::<MeansOfDeath, u32>::with_capacity(1000),
            total_kills: 0,
            players: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct Matches {
    current_match: Match,
    matches: Vec<Match>,
    errors: Vec<String>,
}

impl Matches {
    pub fn new_match(&mut self) {
        let last_match = mem::take(&mut self.current_match);
        self.matches.push(last_match);
    }

    pub fn add_player(&mut self, player: &str) {
        self.current_match.players.push(player.to_string());
    }

    pub fn add_kill(&mut self, killer: &str, means_of_death: MeansOfDeath) {
        self.current_match.total_kills += 1;
        self.current_match
            .kills
            .entry(killer.into())
            .and_modify(|c| *c += 1)
            .or_insert(1);
        self.add_means_of_death(means_of_death);
    }

    pub fn killed_by_world(&mut self, killed: &str, means_of_death: MeansOfDeath) {
        self.current_match.total_kills += 1;
        self.current_match
            .kills
            .entry(killed.into())
            .and_modify(|c| {
                *c = c.checked_sub(1).unwrap_or_default();
            });
        self.add_means_of_death(means_of_death);
    }

    fn add_means_of_death(&mut self, means_of_death: MeansOfDeath) {
        self.current_match
            .means_of_death
            .entry(means_of_death)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    pub fn add_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }

    pub fn all_matches(mut self) -> Vec<Match> {
        self.matches.push(self.current_match);
        self.matches
    }
}

const WORLD: &str = "<world>";
static INIT_GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#".*\d:\d\d InitGame:"#).unwrap());
static PLAYER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#".*\d:\d\d ClientUserinfoChanged: \d+ n\\(?P<player>.*?)\\.*"#).unwrap()
});
static KILL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#".*\d:\d\d (Kill: \d* \d* \d*): (?P<killer>.*) killed (?P<killed>.*) by (?P<means>.*)"#,
    )
    .unwrap()
});

fn main() {
    let path = args_os().nth(1).unwrap_or("res/qgames.log".into());
    //.expect("provide a path to the file as an argument");

    let path = Path::new(&path);
    let file = File::open(path).expect("failed to open file");
    let mapped_data = unsafe { Mmap::map(&file) }.expect("failed to create memory map");

    let raw_data: &[u8] = &mapped_data;

    let rows = raw_data
        .split(|&b| b == b'\n')
        .map(|row| unsafe { std::str::from_utf8_unchecked(row) })
        .collect::<Vec<_>>();

    let matches = rows
        .iter()
        .fold(Matches::default(), |matches, row| {
            analyze_event(row, matches)
        })
        .all_matches();

    for m in matches.into_iter().map(match_report) {
        println!("{m}");
    }
}

/// Analyze event from log row, updating info to current match if necessary.
fn analyze_event(row: &&str, mut matches: Matches) -> Matches {
    match event_from_row(row) {
        Ok(event) => match event {
            LogEvent::NewMatch => matches.new_match(),
            LogEvent::AddPlayer(player) => matches.add_player(&player),
            LogEvent::Kill {
                killer,
                killed,
                means,
            } => {
                if killer == WORLD {
                    matches.killed_by_world(&killed, means);
                } else {
                    matches.add_kill(&killer, means);
                }
            }
            LogEvent::Other => {}
        },
        Err(e) => matches.add_error(&e),
    }
    matches
}

enum LogEvent {
    NewMatch,
    AddPlayer(String),
    Kill {
        killer: String,
        killed: String,
        means: MeansOfDeath,
    },
    Other,
}

/// Extract event from log row.
fn event_from_row(row: &str) -> Result<LogEvent, String> {
    // New match
    if INIT_GAME_REGEX.is_match(row) {
        return Ok(LogEvent::NewMatch);
    }
    // Logged player
    if let Some(captures) = PLAYER_REGEX.captures(row) {
        if let Some(player) = captures.name("player") {
            return Ok(LogEvent::AddPlayer(player.as_str().to_string()));
        }
    }
    // Kill player
    if let Some(captures) = KILL_REGEX.captures(row) {
        if let (Some(killer), Some(killed), Some(means)) = (
            captures.name("killer"),
            captures.name("killed"),
            captures.name("means"),
        ) {
            match MeansOfDeath::try_from(means.as_str()) {
                Ok(means) => {
                    return Ok(LogEvent::Kill {
                        killer: killer.as_str().to_string(),
                        killed: killed.as_str().to_string(),
                        means,
                    });
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

fn match_report(c_match: Match) -> String {
    let mut lines = format!(
        "Match {}\nTotal kills: {}\nRanking:\n",
        c_match.id, c_match.total_kills,
    );
    let mut ranking = c_match
        .kills
        .into_iter()
        .map(|m| (m.0, m.1))
        .collect::<Vec<_>>();
    ranking.sort_by(|a, b| b.1.cmp(&a.1));
    for (i, (player, kills)) in ranking.into_iter().enumerate() {
        lines.push_str(&format!("{} - {player}: {kills}\n", i + 1));
    }

    let mut means = c_match
        .means_of_death
        .into_iter()
        .map(|m| (m.0, m.1))
        .collect::<Vec<_>>();
    means.sort_by(|a, b| b.1.cmp(&a.1));
    for (i, (means, kills)) in means.into_iter().enumerate() {
        lines.push_str(&format!("{} - {means}: {kills}\n", i + 1));
    }

    lines.push('\n');
    lines
}

#[cfg(test)]
mod tests {

    use super::*;
}
