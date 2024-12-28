use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    fn to_char_vec(&self) -> Vec<char> {
        match self {
            BoggleChar::A => return vec!['A'],
            BoggleChar::B => return vec!['B'],
            BoggleChar::C => return vec!['C'],
            BoggleChar::D => return vec!['D'],
            BoggleChar::E => return vec!['E'],
            BoggleChar::F => return vec!['F'],
            BoggleChar::G => return vec!['G'],
            BoggleChar::H => return vec!['H'],
            BoggleChar::I => return vec!['I'],
            BoggleChar::J => return vec!['J'],
            BoggleChar::K => return vec!['K'],
            BoggleChar::L => return vec!['L'],
            BoggleChar::M => return vec!['M'],
            BoggleChar::N => return vec!['N'],
            BoggleChar::O => return vec!['O'],
            BoggleChar::P => return vec!['P'],
            BoggleChar::Qu => return vec!['Q', 'U'],
            BoggleChar::R => return vec!['R'],
            BoggleChar::S => return vec!['S'],
            BoggleChar::T => return vec!['T'],
            BoggleChar::U => return vec!['U'],
            BoggleChar::V => return vec!['V'],
            BoggleChar::W => return vec!['W'],
            BoggleChar::X => return vec!['X'],
            BoggleChar::Y => return vec!['Y'],
            BoggleChar::Z => return vec!['Z'],
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
