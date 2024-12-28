use boggle::BoggleGame;
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
    let mut game = BoggleGame::new(args.size, args.time, args.diagonals, args.dictionary);
    game.start();
}
