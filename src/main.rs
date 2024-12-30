use std::time::Instant;

use boggle::BoggleSolver;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    #[arg(short, long, default_value_t = 90)]
    time: i32,

    #[arg(short, long, default_value_t = 4)]
    size: i32,

    #[arg(short, long, action)]
    diagonals: bool,

    #[arg(long, default_value_t = String::from("dictionary.json"))]
    dictionary: String,
}

fn main() {
    let args = Args::parse();

    let iterations = 1;
    let mut boggle = BoggleSolver::new(args.size, args.diagonals, args.dictionary.clone());

    let mut total_duration = 0;
    for _ in 0..iterations {
        let start = Instant::now();
        boggle.reshuffle();
        let duration = start.elapsed();
        total_duration += duration.as_nanos();
    }

    let average_duration = (total_duration / iterations) as f64 / 1e9;
    println!("Average time elapsed: {} seconds", average_duration);
}
