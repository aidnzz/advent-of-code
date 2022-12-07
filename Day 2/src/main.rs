use std::{mem, str::FromStr};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
enum Hand {
    Rock = 1,
    Paper,
    Scissors,
}

impl Hand {
    #[inline]
    fn score(&self) -> u32 {
        *self as u32
    }
}

#[derive(Clone, Copy)]
struct Round {
    opponent: Hand,
    our: Hand,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum Outcome {
    Won = 6,
    Draw = 3,
    Loss = 0,
}

impl Outcome {
    #[inline]
    fn score(&self) -> u32 {
        *self as u32
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        if self.opponent == self.our {
            return Outcome::Draw;
        }

        match (self.opponent, self.our) {
            (Hand::Paper, Hand::Scissors)
            | (Hand::Rock, Hand::Paper)
            | (Hand::Scissors, Hand::Rock) => Outcome::Won,
            _ => Outcome::Loss,
        }
    }

    fn score(&self) -> u32 {
        self.our.score() + self.outcome().score()
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: &[u8; 3] = s.as_bytes().try_into().ok().ok_or(())?;

        let x = data[0] & 0x3;
        let y = (data[2] & 0xF) - 7;

        if !(x <= 3 && y <= 3) {
            return Err(());
        }

        unsafe {
            Ok(Self {
                opponent: std::mem::transmute::<u8, Hand>(x),
                our: std::mem::transmute::<u8, Hand>(y),
            })
        }
    }
}

fn main() {
    const LINES: &'static str = include_str!("input.txt");

    let total: u32 = LINES
        .lines()
        .map(|l| l.parse::<Round>().unwrap().score())
        .sum();

    println!("Total score: {total}");
}
