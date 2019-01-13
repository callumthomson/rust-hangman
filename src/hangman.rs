use std::io;
use std::mem;

use super::filereader::FileReader;

pub struct Hangman {
    lives: i32,
    dashed_word: Vec<char>,
    word: Vec<char>,
    guesses: Vec<char>,
    pub state: State,
}

pub enum State {
    Playing,
    Lost,
    Won
}

impl Hangman {
    pub fn init() -> Hangman
    {
        let word = Hangman::select_random_word();
        Hangman {
            lives: 10,
            dashed_word: Hangman::generate_dashed_word(&word),
            word,
            guesses: Vec::new(),
            state: State::Playing
        }
    }
    pub fn take_guess(&mut self)
    {
        println!("Guess a letter: ");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let mut input : Vec<char> = input.trim().chars().collect();

        if input.len() != 1 {
            println!("Guesses should be only one character.");
            return
        }

        let guess: char = input.remove(0);

        if self.guess_is_duplicate(&guess) {
            return
        }
        if self.guess_is_correct(&guess) {
            self.replace_dashes_with_character(guess)
        }
        self.guesses.push(guess);
        self.print_dashed_word();

        self.check_lives();
        self.check_complete();
    }
    pub fn guess_is_duplicate(&self, guess: &char) -> bool
    {
        for past_guess in &self.guesses {
            if guess == past_guess {
                println!("You've guessed that already!");
                return true
            }
        }
        false
    }
    pub fn guess_is_correct(&mut self, guess: &char) -> bool
    {
        for c in &self.word {
            if c == guess {
                println!("Correct, you got a letter right!");
                return true
            }
        }
        println!("Wrong, try again!");
        self.lives -= 1;
        false
    }
    pub fn check_lives(&mut self)
    {
        if self.lives == 0 {
            println!("You've run out of guesses.");
            let word_string: &String = &self.word.iter().collect();
            println!("The word was: {}", word_string);
            self.state = State::Lost;
            return;
        }
        println!("Guesses remaining: {}", self.lives);
    }
    fn check_complete(&mut self)
    {
        let mut complete = true;
        for c in &self.dashed_word {
            if c == &'_' {
                complete = false;
            }
        }
        if complete {
            self.state = State::Won
        }
    }
    fn select_random_word() -> Vec<char>
    {
        let mut file_reader = FileReader::init("words.txt");
        file_reader.random_word()
    }
    fn generate_dashed_word(word: &Vec<char>) -> Vec<char>
    {
        let mut dashed: Vec<char> = Vec::new();
        for _ in word {
            dashed.push('_');
            print!("{} ", '_');
        }
        println!();
        dashed
    }
    fn print_dashed_word(&self)
    {
        for c in &self.dashed_word {
            print!("{} ", c);
        }
        println!();
    }
    fn replace_dashes_with_character(&mut self, character: char)
    {
        for (i, c) in self.word.iter().enumerate() {
            if c == &character {
                mem::replace(&mut self.dashed_word[i], character);
            }
        }
    }
}