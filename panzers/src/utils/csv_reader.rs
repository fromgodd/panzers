use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Read};
use csv;

pub fn read_csv_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(BufReader::new(file));

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}