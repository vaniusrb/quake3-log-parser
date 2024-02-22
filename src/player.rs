use std::fmt;

/// New type for player.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Player(String);

impl From<&str> for Player {
    fn from(value: &str) -> Self {
        Player(value.into())
    }
}

impl From<String> for Player {
    fn from(value: String) -> Self {
        Player(value)
    }
}

impl From<Player> for String {
    fn from(value: Player) -> Self {
        value.0
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
