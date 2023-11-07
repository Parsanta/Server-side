#[cfg(test)]
mod tests {
    use crate::lib::*;
    use crate::server_fucntions::*;
    use std::collections::HashMap;
    use std::net::SocketAddr;
    use std::str::FromStr;

    #[test]
    fn test_hangman_game_state() {
        let word = "apple".to_string();
        let max_attempts = 6;
        let mut game = HangmanGameState::new(word.clone(), max_attempts);

        assert_eq!(game.word, word);
        assert_eq!(game.display_word, "_".repeat(word.len()));
        assert_eq!(game.attempts_left, max_attempts);
        assert!(game.guessed_letters.is_empty());

        // Test the process_guess function.
        assert_eq!(game.process_guess('a'), true); // Correct guess
        assert_eq!(game.process_guess('b'), false); // Incorrect guess
        assert_eq!(game.process_guess('a'), false); // Already guessed

        // Check that game state is updated correctly.
        assert_eq!(game.display_word, "a____");
        assert_eq!(game.attempts_left, 5);

        assert_eq!(game.is_game_over(), false);

        game.attempts_left = 0;
        assert_eq!(game.is_game_over(), true);
    }

    // #[test]
    // fn test_update_score() {
    //     let mut game = HangmanGameState::new("apple".to_string(), 6);
    //     let mut clients: HashMap<SocketAddr, u32> = HashMap::new();
    //     let client_address = "127.0.0.1:12345".parse().unwrap();
    //     update_score(&mut game, 'a', &mut clients, client_address);
    //     assert_eq!(clients.get(&client_address), Some(&1));
    // }

    #[test]
    fn test_get_max() {
        let mut clients: HashMap<SocketAddr, u32> = HashMap::new();
        clients.insert(SocketAddr::from_str("127.0.0.1:12345").unwrap(), 3);
        clients.insert(SocketAddr::from_str("192.168.0.1:54321").unwrap(), 5);
        clients.insert(SocketAddr::from_str("10.0.0.1:8080").unwrap(), 2);
        assert_eq!(get_max(&clients), 5);
    }

    #[test]
    fn test_word() {
        let (word, attempts) = word();
        assert!(!word.is_empty());
    }
}
