use crate::{player::Player, MeansOfDeath};

pub enum LogEvent {
    NewMatch,
    AddPlayer(Player),
    Kill { killer: Player, means: MeansOfDeath },
    KilledByWorld { killed: Player, means: MeansOfDeath },
    Other,
}
