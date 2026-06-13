// main.rs — entry point for the blockchain CLI application
//
// Module layout:
//   block       — Block struct and hashing logic
//   blockchain  — Chain management (add, display, validate, tamper)
//   cli         — Interactive REPL and demo runner

mod block;
mod blockchain;
mod cli;

use blockchain::Blockchain;

fn main() {
    println!("Initialising blockchain…");
    let mut chain = Blockchain::new();

    // Hand control to the interactive CLI.
    // Pass `--demo` as the first argument to run the scripted demo and exit.
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--demo") {
        cli::run_demo(&mut chain);
    } else {
        cli::run(&mut chain);
    }
}
