// Import the standard library tools we need.
// ErrorKind is used to detect non-blocking socket read results.
// Read and Write are needed for reading from and writing to sockets.
use std::io::{ErrorKind, Read, Write};
// TcpListener is the server socket type that accepts client connections.
use std::net::TcpListener;
// mpsc provides a channel for sending messages from worker threads to main thread.
use std::sync::mpsc;
// thread is used to spawn a new thread for each client connection.
use std::thread;

// Address and port where the server will listen.
const LOCAL: &str = "127.0.0.1:8080";
// Fixed size for each message buffer.
const MSG_SIZE: usize = 64;

// A small helper function to pause a little bit in the loops.
fn sleep() {
    thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    // Bind the server socket to LOCAL and start listening.
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    println!("Server listening on {}", LOCAL);

    // Make accept() non-blocking so we can continue running the main loop.
    server
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking");

    // Keep a list of connected clients to broadcast messages to them.
    let mut clients = vec![];
    // Channel for client handler threads to send messages back to the main thread.
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        // Try to accept a new client connection.
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            // Each client handler needs its own sender clone.
            let tx = tx.clone();
            // Keep a copy of the socket so we can write back to it later.
            clients.push(socket.try_clone().expect("Failed to clone client"));

            // Spawn a new thread to read messages from this client.
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                // Read exactly MSG_SIZE bytes from the socket.
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        // Convert the received bytes into a UTF-8 string.
                        let msg = buff
                            .into_iter()
                            .take_while(|&x| x != 0)
                            .collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        // Send the received message back to the main thread.
                        tx.send(msg).expect("Failed to send message to main thread");
                    }
                    // If there is no data yet, just continue looping.
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    // If any other error happens, close this client connection.
                    Err(_) => {
                        println!("Closing connection with {}", addr);
                        break;
                    }
                }
                sleep();
            });
        }

        // If the main thread receives a message from any client thread,
        // print it and broadcast it to all connected clients.
        if let Ok(msg) = rx.try_recv() {
            println!("Received from client: {}", msg);
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);
                    match client.write_all(&buff).map(|_| client) {
                        Ok(client) => Some(client),
                        Err(_) => None,
                    }
                })
                .collect::<Vec<_>>();
        }

        // Pause briefly to avoid busy-waiting.
        sleep();
    }
}
