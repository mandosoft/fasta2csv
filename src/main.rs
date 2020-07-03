use std::env;
use std::process;

use fasta2csv::CSV;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = CSV::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = fasta2csv::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
