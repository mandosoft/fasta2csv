use std::fs;
use std::error::Error;
use std::fs::File;
 use std::io::Write;

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

pub fn search_delimiter<'a>(contents: String, filename: String, mut csv: File) {
    let delimiter: &str = ">";
    let mut entry: String = String::new();
    println!("{:?}", entry);
    for line in contents.lines() {
        if line.contains(delimiter) {
            let split = line.split("");
            let entry: String = split.map(|x| x.to_owned() + ",").collect();
            write!(csv, "{}", entry);
            //&entry = line.to_owned();
        }
        else {
            entry.push_str(line);
            println!("{:?}", entry);
        }
    }
}

pub fn run(config: CSV) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let prefix:Vec<&str> = config.filename.split(".").collect();
    let csv = fs::File::create(prefix[0].to_owned() + ".csv").expect("Unable to create file");
    search_delimiter(contents, config.filename, csv);
    Ok(())
}