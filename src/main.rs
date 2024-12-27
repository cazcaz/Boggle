use boggle::Dice;
fn main() {
    let mut board = Dice::new();
    println!("{}", board);
    board.find_all_words();
    let word_set = board.get_possible_words();
    println!("{:?}", word_set);
}
