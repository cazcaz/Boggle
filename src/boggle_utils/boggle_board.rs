use super::{boggle_char::BoggleChar, boggle_dice};
use serde::Serialize;
use std::fmt;

#[derive(Clone, Serialize)]
pub struct BoggleBoard {
    board: Vec<Vec<BoggleChar>>,
    board_size: i32,
}

impl BoggleBoard {
    pub fn new(size: i32) -> Self {
        let full_chars: Vec<BoggleChar> = boggle_dice::scramble_dice(size);
        let mut board: Vec<Vec<BoggleChar>> = vec![];
        for i in 0..size {
            let mut row: Vec<BoggleChar> = vec![];
            for j in 0..size {
                let index: usize = (i * size + j).try_into().unwrap();
                row.push(full_chars[index].clone());
            }
            board.push(row);
        }
        Self {
            board,
            board_size: size,
        }
    }

    pub fn from(chars: Vec<char>, size: i32) -> Self {
        let full_chars: Vec<BoggleChar> = chars
            .into_iter()
            .map(|c| BoggleChar::from(c.to_ascii_uppercase() as u8))
            .collect();
        let mut board: Vec<Vec<BoggleChar>> = vec![];
        for i in 0..size {
            let mut row: Vec<BoggleChar> = vec![];
            for j in 0..size {
                let index: usize = (i * size + j).try_into().unwrap();
                row.push(full_chars[index].clone());
            }
            board.push(row);
        }
        Self {
            board,
            board_size: size,
        }
    }

    pub fn in_bounds(&self, location: &(i32, i32)) -> bool {
        location.0 >= 0
            && location.0 <= self.board_size - 1
            && location.1 >= 0
            && location.1 <= self.board_size - 1
    }

    pub fn access(&self, coords: (usize, usize)) -> BoggleChar {
        self.board[coords.0][coords.1].clone()
    }

    pub fn get_chars(&self) -> Vec<Vec<BoggleChar>> {
        self.board.clone()
    }
}

impl fmt::Display for BoggleBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}
