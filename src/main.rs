use std::time::Instant;

use boggle::{BoggleGame, BoggleSolverInterface};
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

    #[arg(short, long, action)]
    multi_thread: bool,

    #[arg(long, default_value_t = String::from(""))]
    board: String,
}

fn main() {
    let args = Args::parse();
    if args.board.len() == 0 {
        let mut game = BoggleGame::new(
            args.size,
            args.time,
            args.diagonals,
            args.dictionary,
            args.multi_thread,
        );
        game.start();
    } else {
        let start = Instant::now();
        let solver = BoggleSolverInterface::new(
            args.board,
            args.size,
            args.diagonals,
            args.dictionary,
            args.multi_thread,
        );
        let duration = start.elapsed();
        solver.output_words();

        println!(
            "Time taken: {} seconds",
            (duration.as_nanos() as f64) / 1e9 as f64
        );
    }
}
