use rand::Rng;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

use crate::lib::HangmanGameState;
pub fn send_message(count: i32, server_socket: &UdpSocket, max_value: u32, top: Vec<&SocketAddr>) {
    if count == 1 {
        if let Err(e) = server_socket.send_to(
            format!("Game Over!\nYou Won!!!!!\nYour Score was: {}", max_value).as_bytes(),
            top[0],
        ) {
            eprintln!("Error sending message: {}", e);
        }
    } else if count > 1 {
        top.iter().for_each(|i| {
            server_socket
                .send_to(
                    format!("Game Over!\nDraw!!!!!\nYour Score was: {}", max_value).as_bytes(),
                    i,
                )
                .unwrap();
        });
    }
}

pub fn get_max(clients: &HashMap<SocketAddr, u32>) -> u32 {
    let mut max_value = 0;

    for (_key, value) in clients {
        if *value > max_value {
            max_value = *value;
        }
    }
    max_value
}

pub fn update_score(
    game: &mut HangmanGameState,
    letter: char,
    clients: &mut HashMap<SocketAddr, u32>,
    client_address: SocketAddr,
) {
    if game.process_guess(letter) {
        if let Some(score) = clients.get_mut(&client_address) {
            *score += 1;
        } else {
            eprintln!("Error while incrementing user score: User not found");
        }
    }
}

pub fn word() -> (String, u8) {
    let word_list = vec![
        ("apple", 6),
        ("banana", 7),
        ("cherry", 5),
        ("grape", 5),
        ("orange", 6),
        ("strawberry", 9),
        ("watermelon", 10),
        ("apricot", 7),
        ("blueberry", 8),
        ("cantaloupe", 11),
        ("coconut", 7),
        ("fig", 4),
        ("kiwi", 4),
        ("lemon", 5),
        ("mango", 6),
        ("peach", 6),
        ("pear", 4),
        ("pineapple", 9),
        ("plum", 4),
        ("pomegranate", 11),
        ("raspberry", 8),
        ("strawberry", 9),
        ("watermelon", 10),
        ("avocado", 7),
        ("blackberry", 9),
        ("cranberry", 9),
        ("dragonfruit", 11),
        ("guava", 5),
        ("kiwifruit", 9),
        ("lime", 4),
        ("papaya", 6),
        ("persimmon", 9),
        ("raspberry", 8),
        ("tangerine", 8),
        ("tomato", 6),
    ];
    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..word_list.len());
    let (word, attempts) = word_list[index];
    (word.to_string(), attempts)
}
