// Import standard IO tools. `self` is used for stdin reading. `ErrorKind` is used
// to detect non-blocking socket behavior. `Read` and `Write` are needed for socket IO.
use std::io::{self, ErrorKind, Read, Write};
// TcpStream is the client socket type used to connect to the server.
use std::net::TcpStream;
// mpsc channel is used to send user messages from the keyboard loop to the
// socket thread. TryRecvError is used for non-blocking channel reads.
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

// Server address and port to connect to.
const LOCAL: &str = "127.0.0.1:8080";
// Fixed size for each message buffer.
const MSG_SIZE: usize = 64;
fn main() {
    // Try to connect to the server at LOCAL.
    // If the connection fails, print an error and exit.
    let mut client = match TcpStream::connect(LOCAL) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("Failed to connect to server at {}: {}", LOCAL, err);
            return;
        }
    };

    // Set the socket to non-blocking so reads return immediately if there is no data.
    client
        .set_nonblocking(true)
        .expect("Failed to set non-blocking mode");

    // Create a channel so the keyboard loop can send typed messages to the socket thread.
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a thread to handle both reading from the socket and sending outgoing messages.
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];

        // Try to read exactly MSG_SIZE bytes from the server.
        match client.read_exact(&mut buff) {
            Ok(_) => {
                // Convert the received bytes into a string, stopping at the first 0 byte.
                let msg = buff
                    .into_iter()
                    .take_while(|&x| x != 0)
                    .collect::<Vec<_>>();
                println!("Message received: {:?}", msg);
            }
            // If there's no data yet on the socket, ignore this and continue.
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            // Any other error means the server connection is gone.
            Err(_) => {
                println!("Connection with server was severed");
                break;
            }
        }

        // Try to receive a user message from the keyboard loop.
        match rx.try_recv() {
            Ok(msg) => {
                // Convert the message to bytes and pad it to MSG_SIZE.
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Writing to socket failed");
                println!("Message sent: {}", msg);
            }
            // If there is no message waiting, do nothing.
            Err(TryRecvError::Empty) => (),
            // If the sending side is disconnected, stop the thread.
            Err(TryRecvError::Disconnected) => {
                println!("Client Disconnected");
                break;
            }
        }

        // Sleep briefly to avoid busy-waiting.
        thread::sleep(Duration::from_millis(100));
    });

    // Prompt the user to type messages.
    println!("Write a message and press enter to send it");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Reading from stdin failed");
        let msg = buff.trim().to_string();

        // Exit if the user types :quit or if the channel is closed.
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
        
    }
    println!("bye bye");
}
