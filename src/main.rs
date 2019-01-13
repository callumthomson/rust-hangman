mod hangman;
mod filereader;

use self::hangman::Hangman;
use self::hangman::State;

fn main() {
    let mut hangman = Hangman::init();

    loop {
        hangman.take_guess();

        match hangman.state {
            State::Playing => {},
            State::Won =>  {
                println!("You Won");
                break
            },
            State::Lost => {
                println!("You Lost");
                break;
            },
        }
    }
}