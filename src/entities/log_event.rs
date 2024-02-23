use super::{means_of_death::MeansOfDeath, player::Player};

/// Type of log event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogEvent {
    NewMatch,
    AddPlayer(Player),
    Kill { killer: Player, means: MeansOfDeath },
    KilledByWorld { killed: Player, means: MeansOfDeath },
    Other,
}
