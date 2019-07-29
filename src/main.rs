use rust_hangman::{HangMan, HangManState};
use std::fs::File;
use std::io::{Seek, Stdin, stdin, Read};

use text_io::*;

fn main() {
    let mut hangman = HangMan::new("word");

    println!("{}", hangman);

    loop {
        let c: char = read!();

        match hangman.submit_char(c) {
            HangManState::Win => {println!("Win!")},
            HangManState::Loss => {println!("Loss")},
            HangManState::Duplicate(c) => {println!("{} already used", c)},
            HangManState::Normal => {},
        };

        println!("{}", hangman);
    }
}