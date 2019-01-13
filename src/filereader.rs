use std::fs::File;
use std::io::{BufRead, BufReader};

use rand::Rng;

pub struct FileReader
{
    file_name: String,
    words: Vec<String>,
    pub word: Vec<char>,
}

impl FileReader {
    pub fn init(file_name: &str) -> FileReader
    {
        let mut reader = FileReader {
            file_name: String::from(file_name),
            words: Vec::new(),
            word: Vec::new(),
        };
        reader.read();
        reader
    }
    fn read(&mut self)
    {
        let file = File::open(&self.file_name).expect("Unable to open file");
        for line in BufReader::new(file).lines() {
            match line {
                Ok(string) => {
                    self.words.push(string);
                }
                Err(_) => panic!("Issue reading line")
            }
        }
    }
    pub fn random_word(&mut self) -> Vec<char>
    {
        let index = rand::thread_rng().gen_range(0, self.words.len());
        let word = &self.words[index];
        word.chars().collect()
    }
}