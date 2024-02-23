pub mod accumulator;
pub mod entities;
pub mod parser;
pub mod report;

use crate::report::kills_by_means::KillsByMeansReport;
use accumulator::{
    analyzer::MatchAnalyzer, match_ranking::MatchRanking, matches_accumulator::MatchesAccumulator,
};
use clap::Parser;
use memmap2::Mmap;
use mimalloc::MiMalloc;
use parser::regex_parser::RegexParser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use report::{ranking::RankingReport, Report};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser, Debug)]
#[command(version, about = "Quake 3 Arena Server Log Parser")]
struct Args {
    /// Log filename
    #[arg(short, long, default_value = "res/qgames.log")]
    file: PathBuf,
    /// Show kills by means report
    #[arg(long)]
    kbm: bool,
}

fn main() {
    let args = Args::parse();
    let path = args.file;

    // Load file content
    let path = Path::new(&path);
    let file = File::open(path).expect("error to open file");
    let mapped_data = unsafe { Mmap::map(&file) }.expect("error to create memory map");

    // Split buffer into rows
    let rows = &mapped_data
        .split(|&b| b == b'\n')
        .map(|row| unsafe { std::str::from_utf8_unchecked(row) })
        .collect::<Vec<_>>();

    // Parse the log from the row and extract matches list
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

    // Print report
    let report: Box<dyn Report> = if args.kbm {
        Box::new(KillsByMeansReport::new())
    } else {
        Box::new(RankingReport::new())
    };
    println!("{}", report.report(rankings));
}
