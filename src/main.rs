use boggle::{/*BoggleGame,*/ BoggleSolver};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    //#[arg(short, long, default_value_t = 90)]
    //time: i32,
    #[arg(short, long, default_value_t = 4)]
    size: i32,

    #[arg(short, long, action)]
    diagonals: bool,

    #[arg(short, long, default_value_t = 1)]
    runs: i32,

    #[arg(long, default_value_t = String::from("dictionary.json"))]
    dictionary: String,
}

fn main() {
    let args = Args::parse();
    //   let mut game = BoggleGame::new(args.size, args.time, args.diagonals, args.dictionary);
    //  game.start();

    for _ in 0..args.runs {
        println!("!!!START!!!");
        let boggle = BoggleSolver::new(args.size, args.diagonals, args.dictionary.clone());
        let found_words = boggle.get_possible_words();
        let mut found_words_vec: Vec<&String> = found_words.iter().collect();
        found_words_vec.sort_by(|a, b| b.len().cmp(&a.len()));
        for word in found_words_vec {
            println!("{}", word);
        }
        println!("!!!END!!!");
    }
}
