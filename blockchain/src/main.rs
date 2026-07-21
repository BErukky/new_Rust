use std::io::{self, Write};
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut miner_addr).unwrap();

    print!("Difficulty: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut difficulty).unwrap();

    let diff = difficulty.trim().parse::<u32>().expect("we need an integer");
    println!("generating genesis block!");
    let mut chain = blockchain::new_chain(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
        println!("");

        match choice.trim() {
            "1" => {
                print!("sender: ");
                io::stdout().flush().unwrap();
                let mut sender = String::new();
                io::stdin().read_line(&mut sender).unwrap();

                print!("recipient: ");
                io::stdout().flush().unwrap();
                let mut recipient = String::new();
                io::stdin().read_line(&mut recipient).unwrap();

                print!("amount: ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).unwrap();

                let amt = amount.trim().parse::<f32>().expect("we need a number");
                chain.new_transaction(sender.trim().to_string(), recipient.trim().to_string(), amt);
                println!("Transaction Successful!");
            }
            "2" => match chain.mine() {
                Ok(_) => println!("Block Mined Successfully!"),
                Err(e) => println!("Error: {e}"),
            },
            "3" => {
                print!("New Difficulty: ");
                io::stdout().flush().unwrap();
                difficulty.clear();
                io::stdin().read_line(&mut difficulty).unwrap();
                chain.set_difficulty(difficulty.trim().parse::<u32>().expect("we need an integer"));
                println!("Difficulty Changed!");
            }
            "4" => {
                print!("New Reward: ");
                io::stdout().flush().unwrap();
                difficulty.clear();
                io::stdin().read_line(&mut difficulty).unwrap();
                chain.set_reward(difficulty.trim().parse::<f32>().expect("we need a number"));
                println!("Reward Changed!");
            }
            "0" => process::exit(1),
            _ => println!("Invalid input! Try again."),
        }
    }
}
