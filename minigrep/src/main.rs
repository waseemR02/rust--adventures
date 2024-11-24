use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching '{}' in {}...", config.query, config.filepath);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error with application logic: {e}");
        process::exit(1);
    }
}
