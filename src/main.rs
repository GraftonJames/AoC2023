use std::{fs::File, io::BufReader};
mod dayone;
use crate::dayone::day_one;
mod daytwo;
use crate::daytwo::day_two;
mod daythree;
use crate::daythree::day_three;

fn main() {
    day_one();
    day_two();
    day_three();
}

fn get_file_stream(filename: String) -> BufReader<File> {
    let file = File::open(format!("files/{filename}")).unwrap();
    BufReader::new(file)
}
