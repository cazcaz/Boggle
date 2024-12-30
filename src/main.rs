use boggle::{/*BoggleGame,*/ BoggleSolver};
use clap::Parser;
use std::collections::HashMap;

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
    let mut frequency: HashMap<String, u32> = HashMap::new();
    let mut boggle = BoggleSolver::new(args.size, args.diagonals, args.dictionary.clone());

    for i in 0..args.runs {
        boggle.reshuffle();
        println!("Run: {}", i + 1);
        let found_words = boggle.get_possible_words();
        let mut found_words_vec: Vec<&String> = found_words.iter().collect();
        found_words_vec.sort_by(|a, b| b.len().cmp(&a.len()));

        for word in found_words_vec {
            //println!("{}", word);
            let count = frequency.entry(word.clone()).or_insert(0);
            *count += 1;
        }
    }
    println!("");
    let mut freq_vec: Vec<(&String, &u32)> = frequency.iter().collect();
    let mut word_count = 0;
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    for (word, count) in freq_vec {
        if *count > 3 {
            println!("{}: {}", word, count);
            word_count += 1;
        }
        if word_count > 20 {
            break;
        }
    }
}
