use rand::{seq::SliceRandom, Rng};

use super::boggle_char::BoggleChar;

#[derive(Clone)]
struct BoggleDie {
    letters: [BoggleChar; 6],
}

impl BoggleDie {
    fn roll(&self) -> BoggleChar {
        let mut rand = rand::thread_rng();
        self.letters[rand.gen_range(0..6)].clone()
    }
}

pub fn scramble_dice(board_size: i32) -> Vec<BoggleChar> {
    // If 4 or 5, use the official dice
    let mut rand = rand::thread_rng();
    match board_size {
        4 => {
            let mut dice: Vec<BoggleDie> = vec![];
            let mut indices: Vec<i32> = (0..16).collect();
            indices.shuffle(&mut rand);
            for index in indices {
                dice.push(STANDARD_DICE_4X4[index as usize].clone());
            }
            let mut chars: Vec<BoggleChar> = vec![];
            for die in dice {
                chars.push(die.roll());
            }
            return chars;
        }
        5 => {
            let mut dice: Vec<BoggleDie> = vec![];
            let mut indices: Vec<i32> = (0..25).collect();
            indices.shuffle(&mut rand);
            for index in indices {
                dice.push(STANDARD_DICE_5X5[index as usize].clone());
            }
            let mut chars: Vec<BoggleChar> = vec![];
            for die in dice {
                chars.push(die.roll());
            }
            return chars;
        }
        _ => {
            let mut chars: Vec<BoggleChar> = vec![];
            for _ in 0..board_size * board_size {
                chars.push(roll_fake_dice());
            }
            return chars;
        }
    }
}

// For board sizes larger than 5x5, we just create a larger board based on standard boggle letter distributions
fn roll_fake_dice() -> BoggleChar {
    let mut rand = rand::thread_rng();
    let chance = rand.gen_range(1..86);
    if chance <= 19 {
        return BoggleChar::E;
    } else if chance <= 32 {
        return BoggleChar::T;
    } else if chance <= 44 {
        let chance2 = rand.gen_range(1..3);
        if chance2 == 1 {
            return BoggleChar::R;
        } else {
            return BoggleChar::A;
        }
    } else if chance <= 55 {
        let chance2 = rand.gen_range(1..4);
        if chance2 == 1 {
            return BoggleChar::I;
        } else if chance2 == 2 {
            return BoggleChar::N;
        } else {
            return BoggleChar::O;
        }
    } else if chance <= 64 {
        return BoggleChar::S;
    } else if chance <= 70 {
        return BoggleChar::D;
    } else if chance <= 75 {
        let chance2 = rand.gen_range(1..4);
        if chance2 == 1 {
            return BoggleChar::C;
        } else if chance2 == 2 {
            return BoggleChar::H;
        } else {
            return BoggleChar::L;
        }
    } else if chance <= 79 {
        let chance2 = rand.gen_range(1..5);
        if chance2 == 1 {
            return BoggleChar::F;
        } else if chance2 == 2 {
            return BoggleChar::M;
        } else if chance2 == 3 {
            return BoggleChar::U;
        } else {
            return BoggleChar::P;
        }
    } else if chance <= 82 {
        let chance2 = rand.gen_range(1..3);
        if chance2 == 1 {
            return BoggleChar::G;
        } else {
            return BoggleChar::Y;
        }
    } else if chance <= 84 {
        return BoggleChar::W;
    }
    let chance2 = rand.gen_range(1..7);

    if chance2 == 1 {
        return BoggleChar::B;
    } else if chance2 == 2 {
        return BoggleChar::J;
    } else if chance2 == 3 {
        return BoggleChar::K;
    } else if chance2 == 4 {
        return BoggleChar::Qu;
    } else if chance2 == 5 {
        return BoggleChar::V;
    } else if chance2 == 6 {
        return BoggleChar::X;
    }
    BoggleChar::Z
}

const STANDARD_DICE_4X4: [BoggleDie; 16] = [
    BoggleDie {
        letters: [
            BoggleChar::R,
            BoggleChar::I,
            BoggleChar::F,
            BoggleChar::O,
            BoggleChar::B,
            BoggleChar::X,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::I,
            BoggleChar::F,
            BoggleChar::E,
            BoggleChar::H,
            BoggleChar::E,
            BoggleChar::Y,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::D,
            BoggleChar::E,
            BoggleChar::N,
            BoggleChar::O,
            BoggleChar::W,
            BoggleChar::S,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::U,
            BoggleChar::T,
            BoggleChar::O,
            BoggleChar::K,
            BoggleChar::N,
            BoggleChar::D,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::H,
            BoggleChar::M,
            BoggleChar::S,
            BoggleChar::R,
            BoggleChar::A,
            BoggleChar::O,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::L,
            BoggleChar::U,
            BoggleChar::P,
            BoggleChar::E,
            BoggleChar::T,
            BoggleChar::S,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::A,
            BoggleChar::C,
            BoggleChar::I,
            BoggleChar::T,
            BoggleChar::O,
            BoggleChar::A,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::Y,
            BoggleChar::L,
            BoggleChar::G,
            BoggleChar::K,
            BoggleChar::U,
            BoggleChar::E,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::Qu,
            BoggleChar::B,
            BoggleChar::M,
            BoggleChar::J,
            BoggleChar::O,
            BoggleChar::A,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::E,
            BoggleChar::H,
            BoggleChar::I,
            BoggleChar::S,
            BoggleChar::P,
            BoggleChar::N,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::V,
            BoggleChar::E,
            BoggleChar::T,
            BoggleChar::I,
            BoggleChar::G,
            BoggleChar::N,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::B,
            BoggleChar::A,
            BoggleChar::L,
            BoggleChar::I,
            BoggleChar::Y,
            BoggleChar::T,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::E,
            BoggleChar::Z,
            BoggleChar::A,
            BoggleChar::V,
            BoggleChar::N,
            BoggleChar::D,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::R,
            BoggleChar::A,
            BoggleChar::L,
            BoggleChar::E,
            BoggleChar::S,
            BoggleChar::C,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::U,
            BoggleChar::W,
            BoggleChar::I,
            BoggleChar::L,
            BoggleChar::R,
            BoggleChar::G,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::P,
            BoggleChar::A,
            BoggleChar::C,
            BoggleChar::E,
            BoggleChar::M,
            BoggleChar::D,
        ],
    },
];

const STANDARD_DICE_5X5: [BoggleDie; 25] = [
    BoggleDie {
        letters: [
            BoggleChar::Qu,
            BoggleChar::B,
            BoggleChar::Z,
            BoggleChar::J,
            BoggleChar::X,
            BoggleChar::K,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::T,
            BoggleChar::O,
            BoggleChar::U,
            BoggleChar::O,
            BoggleChar::T,
            BoggleChar::O,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::O,
            BoggleChar::V,
            BoggleChar::W,
            BoggleChar::R,
            BoggleChar::G,
            BoggleChar::R,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::A,
            BoggleChar::A,
            BoggleChar::A,
            BoggleChar::F,
            BoggleChar::S,
            BoggleChar::R,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::A,
            BoggleChar::U,
            BoggleChar::M,
            BoggleChar::E,
            BoggleChar::E,
            BoggleChar::G,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::H,
            BoggleChar::H,
            BoggleChar::L,
            BoggleChar::R,
            BoggleChar::D,
            BoggleChar::O,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::N,
            BoggleChar::H,
            BoggleChar::D,
            BoggleChar::T,
            BoggleChar::H,
            BoggleChar::O,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::L,
            BoggleChar::H,
            BoggleChar::N,
            BoggleChar::R,
            BoggleChar::O,
            BoggleChar::D,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::A,
            BoggleChar::F,
            BoggleChar::A,
            BoggleChar::I,
            BoggleChar::S,
            BoggleChar::R,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::Y,
            BoggleChar::I,
            BoggleChar::F,
            BoggleChar::A,
            BoggleChar::S,
            BoggleChar::R,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::T,
            BoggleChar::E,
            BoggleChar::L,
            BoggleChar::P,
            BoggleChar::C,
            BoggleChar::I,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::S,
            BoggleChar::S,
            BoggleChar::N,
            BoggleChar::S,
            BoggleChar::E,
            BoggleChar::U,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::R,
            BoggleChar::I,
            BoggleChar::Y,
            BoggleChar::P,
            BoggleChar::R,
            BoggleChar::H,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::D,
            BoggleChar::O,
            BoggleChar::R,
            BoggleChar::D,
            BoggleChar::L,
            BoggleChar::N,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::C,
            BoggleChar::C,
            BoggleChar::W,
            BoggleChar::N,
            BoggleChar::S,
            BoggleChar::T,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::T,
            BoggleChar::T,
            BoggleChar::O,
            BoggleChar::T,
            BoggleChar::E,
            BoggleChar::M,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::S,
            BoggleChar::C,
            BoggleChar::T,
            BoggleChar::I,
            BoggleChar::E,
            BoggleChar::P,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::E,
            BoggleChar::A,
            BoggleChar::N,
            BoggleChar::D,
            BoggleChar::N,
            BoggleChar::N,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::M,
            BoggleChar::N,
            BoggleChar::N,
            BoggleChar::E,
            BoggleChar::A,
            BoggleChar::G,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::U,
            BoggleChar::O,
            BoggleChar::T,
            BoggleChar::O,
            BoggleChar::W,
            BoggleChar::N,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::A,
            BoggleChar::E,
            BoggleChar::A,
            BoggleChar::E,
            BoggleChar::E,
            BoggleChar::E,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::Y,
            BoggleChar::I,
            BoggleChar::F,
            BoggleChar::P,
            BoggleChar::S,
            BoggleChar::R,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::E,
            BoggleChar::E,
            BoggleChar::E,
            BoggleChar::E,
            BoggleChar::M,
            BoggleChar::A,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::I,
            BoggleChar::T,
            BoggleChar::I,
            BoggleChar::T,
            BoggleChar::I,
            BoggleChar::E,
        ],
    },
    BoggleDie {
        letters: [
            BoggleChar::E,
            BoggleChar::T,
            BoggleChar::I,
            BoggleChar::L,
            BoggleChar::I,
            BoggleChar::C,
        ],
    },
];
