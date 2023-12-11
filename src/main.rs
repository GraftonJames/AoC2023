use std::{fs::File, io::BufReader};
mod dayone;
use crate::dayone::day_one;
mod daytwo;
use crate::daytwo::day_two;
mod daythree;
use crate::daythree::day_three;
mod dayfour;
use crate::dayfour::day_four;
mod dayfive;
use crate::dayfive::day_five;

fn main() {
    day_one();
    day_two();
    day_three();
    day_four();
    day_five();
}

fn get_file_stream(filename: String) -> BufReader<File> {
    let file = File::open(format!("files/{filename}")).unwrap();
    BufReader::new(file)
}
