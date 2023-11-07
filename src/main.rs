use server::lib::*;
use server::server_fucntions::*;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::str;

fn main() -> std::io::Result<()> {
    let server_socket = UdpSocket::bind("0.0.0.0:12345")?;
    // initializing UDP socket so that all clients can connect with it 
    let mut clients: HashMap<SocketAddr, u32> = HashMap::new();
    let mut buffer = [0; 1024];

    //initializing game state
    let (word_to_guess, attempt) = word();
    let mut game = HangmanGameState::new(word_to_guess, attempt);

    loop {
        //receiving the incoming message and client address
        let (received, client_address) = match server_socket.recv_from(&mut buffer) {
            Ok((received, client_address)) => (received, client_address),
            Err(err) => {
                eprintln!("Error receiving data from the client: {}", err);
                continue;
            }
        };

        //storing the client addresses
        if !clients.contains_key(&client_address) {
            clients.insert(client_address, 0);
            let new_user_message = format!(
                "Game Status: {}\nAttempt left: {} Your Score: 0",
                game.display_word, game.attempts_left
            );
            if let Err(err) = server_socket.send_to(new_user_message.as_bytes(), client_address) {
                eprintln!("Error sending data to the client: {}", err);
            }
            continue;
        }

        let received_data = &buffer[..received];
        let message = match str::from_utf8(received_data) {
            Ok(msg) => msg.chars().next(),
            Err(e) => {
                eprintln!("Error decoding message: {}", e);
                continue;
            }
        };

        //broadcasting the game updates to all the clients
        if let Some(letter) = message {
            update_score(&mut game, letter, &mut clients, client_address);
            for (key, value) in &clients {
                if !(client_address == *key) {
                    let new_user_message = format!("Game Status: {}\nAttempt left: {} Your Score: {}", game.display_word, game.attempts_left, value);
                    if let Err(err) = server_socket.send_to(new_user_message.as_bytes(), *key) {
                        eprintln!("Error sending data to the client: {}", err);
                    }
                }
            }
        }

        //game over logic broadcasting to all the clients
        if game.is_game_over() {
            let max_value = get_max(&clients);
            let mut count = 0;
            let mut top = Vec::new();
            for (key, value) in &clients {
                if *value == max_value {
                    count += 1;
                    top.push(key);
                } else {
                    if let Err(err) = server_socket.send_to(
                        format!("Game Over!\nYou Lost :(\nYour Score was: {}", *value).as_bytes(),
                        key.clone(),
                    ) {
                        eprintln!("Error sending data to the client: {}", err);
                    }
                }
            }
            send_message(count, &server_socket, max_value, top);
            break;
        }

        let new_user_message = format!(
            "Game Status: {}\nAttempt left: {} Your Score: {}",
            game.display_word,
            game.attempts_left,
            clients.get(&client_address).unwrap_or(&0)
        );

        if let Err(err) = server_socket.send_to(new_user_message.as_bytes(), client_address) {
            eprintln!("Error sending data to the client: {}", err);
        }
    }

    Ok(())
}
