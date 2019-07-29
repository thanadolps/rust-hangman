use std::fmt::{Display, Formatter, Error};
use std::collections::HashSet;

pub enum HangManState {
    Win,
    Loss,
    Duplicate(char),
    Normal,
}

pub struct HangMan {
    correct_word: String,
    correctness_mask: Box<[bool]>,
    live: u32,
    used_char: HashSet<char>,
    done: bool,
}

impl HangMan {
    pub fn new(word: impl Into<String>) -> Self {
        let word_str = word.into();
        let mask = vec![false; word_str.len()].into_boxed_slice();
        HangMan {
            correct_word: word_str,
            correctness_mask: mask,
            live: 5,
            used_char: HashSet::new(),
            done: false,
        }
    }

    pub fn submit_char(&mut self, c: char) -> HangManState {

        if self.done {panic!("Game Already Ended")}

        let not_used = self.used_char.insert(c);

        if !not_used {return HangManState::Duplicate(c);}

        if self.correct_word.contains(c) {
            // correct word
            let mask  = &mut self.correctness_mask;
            self.correct_word
                .chars()
                .enumerate()
                .filter(|(i, wc)| *wc==c)
                .for_each(|(i, _)| mask[i] = true);

            if !mask.contains(&false) {
                // correctly guess all character
                self.done = true;
                return HangManState::Win;
            }
        }
        else {
            // wrong word
            if self.live > 1 {
                self.live -= 1;
            }
            else {
                self.done = true;
                return HangManState::Loss;
            }
        }

        HangManState::Normal
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Display for HangMan {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "live: {}", self.live)?;
        writeln!(f, "used: {:?}", self.used_char)?;

        for (correct_char, &correct_mask) in self.correct_word.chars().zip(self.correctness_mask.iter()) {
            if correct_mask {
                write!(f, "{}", correct_char)?;
            }
            else {
                write!(f, "_")?;
            }
            write!(f, " ")?;
        }

        Ok(())
    }
}