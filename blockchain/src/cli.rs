// cli.rs — interactive command-line interface

use crate::blockchain::Blockchain;
use std::io::{self, Write};

/// Launches the interactive REPL loop.
pub fn run(blockchain: &mut Blockchain) {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║        Rust Blockchain — Interactive CLI     ║");
    println!("╠══════════════════════════════════════════════╣");
    println!("║  add <data>   — Add a new block              ║");
    println!("║  display      — Show all blocks              ║");
    println!("║  validate     — Validate the chain           ║");
    println!("║  tamper <i> <data> — Tamper with block i     ║");
    println!("║  demo         — Run full demo automatically  ║");
    println!("║  quit         — Exit                         ║");
    println!("╚══════════════════════════════════════════════╝\n");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF — exit gracefully (e.g. piped input exhausted)
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Split into command + remainder
        let (cmd, rest) = match input.find(' ') {
            Some(pos) => (&input[..pos], input[pos + 1..].trim()),
            None => (input, ""),
        };

        match cmd {
            "add" => {
                if rest.is_empty() {
                    println!("Usage: add <transaction data>");
                } else {
                    blockchain.add_block(rest.to_string());
                }
            }

            "display" => {
                blockchain.display();
            }

            "validate" => {
                print_validation(blockchain.validate());
            }

            "tamper" => {
                // Expected format: tamper <index> <new data>
                let parts: Vec<&str> = rest.splitn(2, ' ').collect();
                if parts.len() < 2 {
                    println!("Usage: tamper <block_index> <new data>");
                } else {
                    match parts[0].parse::<usize>() {
                        Ok(idx) => {
                            if let Err(e) = blockchain.tamper(idx, parts[1].to_string()) {
                                println!("Error: {}", e);
                            }
                        }
                        Err(_) => println!("Error: block index must be a non-negative integer."),
                    }
                }
            }

            "demo" => {
                run_demo(blockchain);
            }

            "quit" | "exit" | "q" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Unknown command '{}'. Type 'demo' to see a walkthrough.", cmd);
            }
        }
    }
}

/// Runs a scripted demo that exercises every feature end-to-end.
pub fn run_demo(blockchain: &mut Blockchain) {
    println!("\n━━━━━━━━━━━━━━━━━━━━ DEMO START ━━━━━━━━━━━━━━━━━━━━");

    // Add sample transactions
    println!("\n[1] Adding transaction blocks…");
    blockchain.add_block("Alice sent 100 tokens to Bob".to_string());
    blockchain.add_block("Bob sent 50 tokens to Carol".to_string());
    blockchain.add_block("Carol sent 25 tokens to Dave".to_string());

    // Display
    println!("\n[2] Displaying the blockchain…");
    blockchain.display();

    // Validate clean chain
    println!("[3] Validating the untampered chain…");
    print_validation(blockchain.validate());

    // Tamper with block 1
    println!("\n[4] Tampering with block #1…");
    blockchain
        .tamper(1, "Alice sent 999999 tokens to Bob".to_string())
        .unwrap();

    // Display after tamper
    println!("\n[5] Displaying the chain after tampering…");
    blockchain.display();

    // Validate tampered chain — should fail
    println!("[6] Validating the tampered chain (expect INVALID)…");
    print_validation(blockchain.validate());

    println!("━━━━━━━━━━━━━━━━━━━━ DEMO END ━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// Prints a human-readable VALID / INVALID result.
fn print_validation(result: Result<(), String>) {
    match result {
        Ok(_) => println!("✅  Blockchain is VALID"),
        Err(reason) => println!("❌  Blockchain is INVALID\n    Reason: {}", reason),
    }
}
