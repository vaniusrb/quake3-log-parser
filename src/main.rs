use ahash::{HashMap, HashMapExt};
use memmap2::Mmap;
use mimalloc::MiMalloc;
use rayon::{iter::ParallelIterator, slice::*};
use regex::Regex;
use std::{env::args_os, fs::File, path::Path};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const INIT_GAME: &[u8] = b"InitGame:";

enum Kill {
    Killer(String),
    ByWorld(String),
}

struct Match {
    total_kills: u32,
    players: Vec<String> // ["Dono da bola", "Isgalamido", "Zeh"],
    kills: HashMap<String, u32>, // "Dono da bola": 5, "Isgalamido": 18, "Zeh": 20 }
}

impl Match {
    pub fn add_kill(&mut self, killer: &str) {
        //self.kills.
    }

    pub fn killed_by_world(&mut self, killed: &str) {
        //self.kills.
    }

}

const KILL_REGEX: &str = r#".*\d:\d\d (Kill: \d* \d* \d*): (?P<killer>.*) killed (?P<killed>.*) by (?P<cause>.*)"#;

const INIT_GAME_REGEX: &str = r#".*\d:\d\d (InitGame:)"#;

fn main() {
    let kill_regex = Regex::new(KILL_REGEX).unwrap();
    let init_regex = Regex::new(INIT_GAME_REGEX).unwrap();


    let mut matches: HashMap::new();
    let path = args_os().nth(1).unwrap_or("res/qgames.log".into());
    //.expect("provide a path to the file as an argument");

    let path = Path::new(&path);
    let file = File::open(path).expect("failed to open file");
    let mapped_data = unsafe { Mmap::map(&file) }.expect("failed to create memory map");

    let raw_data = &*mapped_data;

    let raw_data = raw_data.strip_suffix(b"\n").unwrap_or(raw_data);

    let rows = raw_data
        //
        // .par_split(|&b| b == INIT_GAME)
        .split(|&b| b == b'\n')
        .map(|row| unsafe { std::str::from_utf8_unchecked(row) })
        .collect::<Vec<_>>();

    let mut current_match = 0;
    for row in rows {
        if init_regex.is_match(row) {
            current_match = Some()
        }
        println!("{row}");
    }

    // let (city, sample) = row.split_once(|&b| b == b';').expect("no ; separator");
    // let sample: Value = fast_parse(sample);
    // (city, sample)
}
