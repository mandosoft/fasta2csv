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

pub fn comma_separate(entry: &str) -> Vec<&str>{

    let split = entry.split("");
    let entry = split.collect::<Vec<&str>>();
    println!("{:?}", entry);

    return entry;

}


pub fn search_delimiter<'a>(contents: String, filename: String) {

    let delimiter: &str = ">";
    let mut entry: String = filename.to_owned();

    for line in contents.lines() {

        if line.contains(delimiter) {
            comma_separate(&entry);
            println!("{}", entry);
            entry = line.to_owned();
        }
        else {
            entry.push_str(line);
        }
    }
}


pub fn run(config: CSV) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    search_delimiter(contents, config.filename);

    Ok(())
}