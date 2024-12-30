use serde::Serialize;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub enum BoggleChar {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Qu,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl BoggleChar {
    pub fn append_to(&self, string: &mut String) {
        for c in self.to_char_vec() {
            string.push(c);
        }
    }

    pub fn to_char_vec(&self) -> Vec<char> {
        match self {
            BoggleChar::A => return vec!['a'],
            BoggleChar::B => return vec!['b'],
            BoggleChar::C => return vec!['c'],
            BoggleChar::D => return vec!['d'],
            BoggleChar::E => return vec!['e'],
            BoggleChar::F => return vec!['f'],
            BoggleChar::G => return vec!['g'],
            BoggleChar::H => return vec!['h'],
            BoggleChar::I => return vec!['i'],
            BoggleChar::J => return vec!['j'],
            BoggleChar::K => return vec!['k'],
            BoggleChar::L => return vec!['l'],
            BoggleChar::M => return vec!['m'],
            BoggleChar::N => return vec!['n'],
            BoggleChar::O => return vec!['o'],
            BoggleChar::P => return vec!['p'],
            BoggleChar::Qu => return vec!['q', 'u'],
            BoggleChar::R => return vec!['r'],
            BoggleChar::S => return vec!['s'],
            BoggleChar::T => return vec!['t'],
            BoggleChar::U => return vec!['u'],
            BoggleChar::V => return vec!['v'],
            BoggleChar::W => return vec!['w'],
            BoggleChar::X => return vec!['x'],
            BoggleChar::Y => return vec!['y'],
            BoggleChar::Z => return vec!['z'],
        }
    }
}

impl fmt::Display for BoggleChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoggleChar::A => write!(f, "A"),
            BoggleChar::B => write!(f, "B"),
            BoggleChar::C => write!(f, "C"),
            BoggleChar::D => write!(f, "D"),
            BoggleChar::E => write!(f, "E"),
            BoggleChar::F => write!(f, "F"),
            BoggleChar::G => write!(f, "G"),
            BoggleChar::H => write!(f, "H"),
            BoggleChar::I => write!(f, "I"),
            BoggleChar::J => write!(f, "J"),
            BoggleChar::K => write!(f, "K"),
            BoggleChar::L => write!(f, "L"),
            BoggleChar::M => write!(f, "M"),
            BoggleChar::N => write!(f, "N"),
            BoggleChar::O => write!(f, "O"),
            BoggleChar::P => write!(f, "P"),
            BoggleChar::Qu => write!(f, "Qu"),
            BoggleChar::R => write!(f, "R"),
            BoggleChar::S => write!(f, "S"),
            BoggleChar::T => write!(f, "T"),
            BoggleChar::U => write!(f, "U"),
            BoggleChar::V => write!(f, "V"),
            BoggleChar::W => write!(f, "W"),
            BoggleChar::X => write!(f, "X"),
            BoggleChar::Y => write!(f, "Y"),
            BoggleChar::Z => write!(f, "Z"),
        }
    }
}

impl From<u8> for BoggleChar {
    fn from(c: u8) -> Self {
        match c as char {
            'A' => BoggleChar::A,
            'B' => BoggleChar::B,
            'C' => BoggleChar::C,
            'D' => BoggleChar::D,
            'E' => BoggleChar::E,
            'F' => BoggleChar::F,
            'G' => BoggleChar::G,
            'H' => BoggleChar::H,
            'I' => BoggleChar::I,
            'J' => BoggleChar::J,
            'K' => BoggleChar::K,
            'L' => BoggleChar::L,
            'M' => BoggleChar::M,
            'N' => BoggleChar::N,
            'O' => BoggleChar::O,
            'P' => BoggleChar::P,
            'Q' => BoggleChar::Qu,
            'R' => BoggleChar::R,
            'S' => BoggleChar::S,
            'T' => BoggleChar::T,
            'U' => BoggleChar::U,
            'V' => BoggleChar::V,
            'W' => BoggleChar::W,
            'X' => BoggleChar::X,
            'Y' => BoggleChar::Y,
            'Z' => BoggleChar::Z,
            _ => panic!("Invalid character"), // Handle invalid characters
        }
    }
}
