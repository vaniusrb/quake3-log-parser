use crate::MeansOfDeath;

pub enum LogEvent {
    NewMatch,
    AddPlayer(String),
    Kill { killer: String, means: MeansOfDeath },
    KilledByWorld { killed: String, means: MeansOfDeath },
    Other,
}
