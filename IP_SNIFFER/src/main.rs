// Import the tools we need from the Rust standard library.
use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{self, Sender};
use std::thread;

// The highest port number we will try.
const MAX_PORT: u16 = 65535;

// A small structure to hold the values we parse from the command line.
struct Arguments {
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    // Parse the command-line arguments and make sure they are valid.
    fn new(args: &[String]) -> Result<Self, &'static str> {
        // If the user gave no IP address, show an error.
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        // If the user asked for help, print usage and stop.
        if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
            println!("Usage: cargo run -- <ip> [threads]");
            return Err("help");
        }

        // If there are too many arguments, show an error.
        if args.len() > 3 {
            return Err("too many arguments");
        }

        // Parse the IP address from the first argument.
        let ipaddr = IpAddr::from_str(&args[1]).map_err(|_| "invalid ip address")?;

        // Parse the optional thread count from the second argument.
        let threads = if args.len() == 3 {
            args[2].parse::<u16>().map_err(|_| "invalid number of threads")?
        } else {
            4
        };

        // Return the parsed values.
        Ok(Self { ipaddr, threads })
    }
}

fn scan(tx: Sender<u16>, start_port: u16, ipaddr: IpAddr, step: u16) {
    // Start scanning from the first port for this thread.
    let mut port = start_port;

    // Keep checking ports until we reach the max port number.
    while port <= MAX_PORT {
        // Try to connect to this port.
        if TcpStream::connect((ipaddr, port)).is_ok() {
            // If the connection works, print a dot and send the port number.
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }

        // Move to the next port for this thread.
        let next_port = u32::from(port) + u32::from(step);
        if next_port > u32::from(MAX_PORT) {
            break;
        }
        port = next_port as u16;
    }
}

fn main() {
    // Collect the command-line arguments into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // Save the program name so we can show it in error messages.
    let program = args[0].clone();

    // Parse the arguments. If parsing fails, print an error and exit.
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err == "help" {
            process::exit(0);
        }

        eprintln!("{} problem parsing arguments: {}", program, err);
        process::exit(0);
    });

    // Make sure we use at least one thread.
    let num_threads = arguments.threads.max(1);

    // Create a channel so threads can send open ports back to the main thread.
    let (tx, rx) = mpsc::channel();

    // Start one scanning thread for each thread we want to use.
    for i in 0..num_threads {
        let tx = tx.clone();
        let ipaddr = arguments.ipaddr;
        thread::spawn(move || {
            // Each thread scans a different starting port.
            scan(tx, i + 1, ipaddr, num_threads);
        });
    }

    // Drop the original sender so the receiver knows no more messages will come.
    drop(tx);

    // Collect all the open ports received from the threads.
    let mut out = Vec::new();
    for port in rx {
        out.push(port);
    }

    // Print a newline after the scanning dots.
    println!("");

    // Sort the open ports so they appear in order.
    out.sort_unstable();

    // Print each open port.
    for port in out {
        println!("{} is open", port);
    }
}
