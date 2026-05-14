//! End-to-end demo: pick a config, init, run, close. Done.
//!
//! Build + run:
//!   cargo run -p poc-06-min-launcher --example launcher
//!   cargo run -p poc-06-min-launcher --example launcher -- my-config.nuna

use poc_06_min_launcher::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "configs/default.nuna".to_string());

    println!("[shell] launcher starting with config '{}'", config);
    let mut rt = Runtime::init(&config)?;

    println!("[shell] handing control to engine...");
    let exit_code = rt.run();
    println!("[shell] engine returned exit code {}", exit_code);

    println!("[shell] launcher done");
    std::process::exit(exit_code);
}
