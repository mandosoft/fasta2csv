use std::fs;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

}

pub struct CSV {
    pub filename: String
}

impl CSV {
    pub fn new(args: &[String]) -> Result<CSV, &'static str> {
        if args.len() > 2 {
            return Err("Too many arguments provided");
        }

        let filename = args[1].clone();

        Ok(CSV {filename})
    }
}

pub fn search_delimiter<'a>(line: &str) -> &str {

    let delimiter: &str = ">";

    if line.contains(delimiter) {
        println!("{}", line)
        }

    return line;
}


pub fn run(config: CSV) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in contents.lines() {
        search_delimiter(line);
    }

    Ok(())
}