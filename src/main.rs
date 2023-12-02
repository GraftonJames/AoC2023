use std::{fs::File,str, io::{BufReader, BufRead}, task::Wake};

fn main() {
    day_one();
}

fn day_one() {
    let stream = get_file_stream(String::from("calibration"));
    let result = stream.lines().map(|l| select_only_digits(l.unwrap())).fold(0, |acc,x| acc + get_first_and_last_digits(x));
    print!("{result}");
}

fn select_only_digits(input: String) -> String{
    input.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn get_file_stream(filename: String) -> BufReader<File> {
    let file = File::open(
        format!("files/{filename}"
    )).unwrap();
    BufReader::new(file)
}

fn get_first_and_last_digits(input: String) -> u32 {
    print!("digits: {input}\n");
    let as_digits:Vec<u32>  = input.as_str().chars().map_while(|c| c.to_digit(10)).collect();
    let mut iter = as_digits.iter();
    while let Some(n) = iter.next() {
        print!("a digit: {n}\n");
    }
    let first = as_digits.first().unwrap();
    let last: u32;
    if let Some(n) = as_digits.last() {
        last = *n;
    } else {
        last = *first;
    }
    first * 10 + last
}
