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

pub fn comma_separate(entry: &str) -> Vec<&str>{
    let split = entry.split("");
    let entry = split.collect::<Vec<&str>>();
    return entry;
}

pub fn write2csv(input_file: &String, entry: &String){
    let prefix:Vec<&str> = input_file.split(".").collect();
    let mut csv = fs::File::create(prefix[0].to_owned() + ".csv").expect("Unable to create file");;
    csv.write_all(entry.as_bytes()).expect("Unable to write data");
}

pub fn search_delimiter<'a>(contents: String, filename: String) {
    let delimiter: &str = ">";
    let mut entry: String = filename.to_owned();
    for line in contents.lines() {
        if line.contains(delimiter) {
            comma_separate(&entry);
            write2csv(&filename, &entry);
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
    let file_input = config.filename.clone();
    search_delimiter(contents, config.filename);
    Ok(())
}