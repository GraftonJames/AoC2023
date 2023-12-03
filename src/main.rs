use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    day_one();
    day_two();
}

fn day_two() {

}

fn day_one() {
    let stream = get_file_stream(String::from("calibration"));
    let result = stream.lines().map(
        |l| select_only_digits(l.unwrap())
    ).fold(
        0, |acc,x| acc + get_first_and_last_digits(x)
    );
    print!("dayone answer: {result}\n");
}

fn select_only_digits(input: String) -> Vec<u32> {
    let numbers = HashMap::from([
        ("zero",0),("0",0),
        ("one",1),("1",1),
        ("two",2),("2",2),
        ("three",3),("3",3),
        ("four",4),("4",4),
        ("five",5),("5",5),
        ("six",6),("6",6),
        ("seven",7),("7",7),
        ("eight",8),("8",8),
        ("nine",9),("9",9)
    ]);
    let mut number_indices: HashMap<usize,u32> = HashMap::new();

    for (word,number) in numbers {
        let matches: Vec<(usize, &str)> = input.match_indices(word).collect();
        for (index,_) in matches {
            number_indices.insert(index, number);
        }
    }
    
    let mut as_vec: Vec<(usize, u32)> = number_indices.into_iter().collect();
    as_vec.sort_by(|(k1, _),(k2, _)| k1.cmp(k2));
    as_vec.into_iter().map(
        |(_, number)| number
    )
    .collect()
}

fn get_file_stream(filename: String) -> BufReader<File> {
    let file = File::open(
        format!("files/{filename}"
    )).unwrap();
    BufReader::new(file)
}

fn get_first_and_last_digits(input: Vec<u32>) -> u32 {
    input.first().unwrap() * 10 + input.last().unwrap()
}
