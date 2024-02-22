pub mod analyzer;
pub mod log_event;
pub mod matches;
pub mod means_of_death;
pub mod parser;
pub mod regex_parser;
pub mod report;
pub mod single_match;

use analyzer::Analyzer;
use matches::Matches;
use means_of_death::MeansOfDeath;
use memmap2::Mmap;
use mimalloc::MiMalloc;
use regex_parser::RegexParser;
use report::match_report;
use std::{env::args_os, fs::File, path::Path};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let path = args_os().nth(1).unwrap_or("res/qgames.log".into());
    //.expect("provide a path to the file as an argument");

    let path = Path::new(&path);
    let file = File::open(path).expect("failed to open file");
    let mapped_data = unsafe { Mmap::map(&file) }.expect("failed to create memory map");

    let rows = &mapped_data
        .split(|&b| b == b'\n')
        .map(|row| unsafe { std::str::from_utf8_unchecked(row) })
        .collect::<Vec<_>>();

    let matches = rows
        .iter()
        .fold(
            Analyzer::new(RegexParser::default(), Matches::default()),
            |analyzer, row| analyzer.digest(row),
        )
        .matches()
        .all_matches();

    for m in matches.into_iter().map(match_report) {
        println!("{m}");
    }
}

#[cfg(test)]
mod tests {

    use super::*;
}
