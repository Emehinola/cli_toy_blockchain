extern crate serde_derive;

mod blockchain;

use std::io;
use std::process;
use std::io::Write;

use  blockchain::Chain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut miner_addr);

    print!("Difficulty: ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut difficulty);

    let diff = difficulty.trim().parse::<u32>().expect("Integer expected");
    println!("Generating genesis block...");

    let mut chain = Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("\nMenu");
        println!("1) New Transaction");
        println!("2) Mine New Block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");

        let _ = io::stdout().flush();
        choice.clear();
        let _ = io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("...Exiting");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut sender);

                print!("Enter receiver address: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut receiver);

                print!("Enter amount: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse().unwrap());

                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed")
                }

            },
            2 => {
                println!("Generating block...");
                let res = chain.generate_new_block();

                match res {
                    true => println!("Block generated successfully"),
                    false=> println!("Block generation failed")
                }
            },
            3 => {
                let mut  new_diff = String::new();
                print!("Enter new difficulty: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut new_diff);

                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());

                match res {
                    true => println!("Diffilty updated successfully"),
                    false => println!("Failed to update difficulty")
                }
            },
            4 => {
                let mut reward = String::new();
                print!("Enter reward: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut reward);

                let res = chain.update_reward(reward.trim().parse().unwrap());

                match res {
                    true => println!("Reward updated successfully"),
                    false => println!("Failed to update reward")
                }
            }
            _ => println!("\tInvalid option.\t")
        }
    }
}
