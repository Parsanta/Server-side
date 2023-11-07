pub mod server_fucntions;
pub mod server_unit_test;
pub mod lib {
    use std::collections::HashSet;
    #[derive(Debug, Clone)]
    pub enum ClientMessage {
        GuessLetter(char),
    }

    #[derive(Debug, Clone)]
    pub struct HangmanGameState {
        pub word: String,
        pub display_word: String,
        pub attempts_left: u8,
        pub guessed_letters: HashSet<char>,
    }

    impl HangmanGameState {
        pub fn new(word: String, max_attempts: u8) -> Self {
            let display_word = "_".repeat(word.len());
            Self {
                word,
                display_word,
                attempts_left: max_attempts,
                guessed_letters: HashSet::new(),
            }
        }

        pub fn is_game_over(&self) -> bool {
            self.display_word == self.word || self.attempts_left == 0
        }

        pub fn process_guess(&mut self, letter: char) -> bool {
            if !self.guessed_letters.contains(&letter) {
                self.guessed_letters.insert(letter);
                if !self.word.contains(letter) {
                    self.attempts_left -= 1;
                    return false;
                } else {
                    for (i, c) in self.word.chars().enumerate() {
                        if c == letter {
                            self.display_word
                                .replace_range(i..i + 1, &letter.to_string());
                        }
                    }
                    return true;
                }
            }
            false
        }
    }
}
