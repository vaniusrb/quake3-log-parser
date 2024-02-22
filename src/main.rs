pub mod accumulator;
pub mod entities;
pub mod parser;
pub mod report;

use accumulator::{
    analyzer::MatchAnalyzer, match_ranking::MatchRanking, matches_accumulator::MatchesAccumulator,
};
use clap::Parser;
use memmap2::Mmap;
use mimalloc::MiMalloc;
use parser::regex_parser::RegexParser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use report::{formatted_report::FormattedReport, Report};
use std::{
    env::args_os,
    fs::File,
    path::{Path, PathBuf},
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Log filename
    #[arg(short, long, default_value = "res/qgames.log")]
    file: PathBuf,
    /// Show Means of death
    #[arg(long)]
    mod_: bool,
}

fn main() {
    let args = Args::parse();
    let path = args.file;
    //.expect("provide a path to the file as an argument");

    let path = Path::new(&path);
    let file = File::open(path).expect("failed to open file");
    let mapped_data = unsafe { Mmap::map(&file) }.expect("failed to create memory map");

    let rows = &mapped_data
        .split(|&b| b == b'\n')
        .map(|row| unsafe { std::str::from_utf8_unchecked(row) })
        .collect::<Vec<_>>();

    // Extract matches list from log rows
    let matches_list = rows
        .iter()
        .fold(
            MatchAnalyzer::new(RegexParser::default(), MatchesAccumulator::default()),
            |analyzer, row| analyzer.digest(row),
        )
        .matches();

    // Sort ranking with parallelism
    let rankings = matches_list
        .all_matches()
        .into_par_iter()
        .map(MatchRanking::new)
        .collect::<Vec<_>>();

    let report: Box<dyn Report> = Box::new(FormattedReport::new());

    println!("{}", report.report(rankings));
}

#[cfg(test)]
mod tests {

    use super::*;
}
