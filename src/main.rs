use std::cmp;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    day_one();
    day_two();
    day_three();
}

#[derive(PartialEq, Clone)]
enum Engine {
    Empty,
    Part(Part),
    Digit(u32),
}

#[derive(PartialEq, Clone)]
enum Part {
    Gear,
    Other,
}

#[derive(PartialEq, Clone)]
struct SchemeNum {
    x: usize,
    y: usize,
    len: usize,
    engine: Engine,
}

fn day_three() {
    let stream = get_file_stream(String::from("engine_scheme"));
    let scheme: Vec<Vec<Engine>> = stream
        .lines()
        .map(|l| map_engine_string_to_engine(l.unwrap()))
        .collect();

    let gear_scheme = scheme.clone();

    let scheme = collect_scheme(scheme);

    let gear_scheme = collect_gear_scheme(gear_scheme);

    let mut sum: u32 = 0;
    let height = scheme.len() - 1;
    for line_index in 0..=height {
        let (above, below) = (
            move_index_and_clamp(line_index, 1, height),
            move_index_and_clamp(line_index, -1, height),
        );

        let gear_indices: Vec<usize> = gear_scheme[line_index].iter().map(|eg| eg.y).collect();

        let mut nums = scheme[line_index].clone();
        let mut above = scheme[above].clone();
        let mut below = scheme[below].clone();


        nums.append(&mut above);
        nums.append(&mut below);

        let nums: Vec<(usize, usize, u32)> = nums.iter()
            .map(|sn| {
                if let Engine::Digit(number) = sn.engine {
                    return (sn.y, sn.len, number);
                } else {
                    panic!();
                }
            })
            .collect();

        for g in gear_indices {
            let mut adj_nums = vec![];
            for n in nums.iter() {
                if g + 1 >= n.0 && g - 1 <= n.0 + n.1 - 1 {
                    adj_nums.push(n.2);
                }
            }
            if adj_nums.len() == 2 {
                let n1 = adj_nums.pop().unwrap();
                print!("{}, ", n1);
                let n2 = adj_nums.pop().unwrap();
                print!("{}\n", n2);
                sum += n1 * n2;
            }
        }

        
    }

    print!("day three: {}", sum);
}

fn move_index_and_clamp(i: usize, delta: isize, max: usize) -> usize {
    if delta > 0 {
        (i.checked_add_signed(delta).unwrap()).clamp(0, max)
    } else if delta < 0 {
        i.saturating_add_signed(delta)
    } else {
        i
    }
}

fn collect_gear_scheme(scheme: Vec<Vec<Engine>>) -> Vec<Vec<SchemeNum>> {
    scheme
        .into_iter()
        .enumerate()
        .map(|(i, l)| collect_gear_from_scheme_line(l, i))
        .collect()
}

fn collect_gear_from_scheme_line(scheme_line: Vec<Engine>, index: usize) -> Vec<SchemeNum> {
    let mut iter = scheme_line.iter().enumerate();
    let mut output = vec![];

    while let Some(e) = iter.find(|&e| match e.1 {
        Engine::Part(Part::Gear) => true,
        _ => false,
    }) {
        output.push(SchemeNum {
            x: index,
            y: e.0,
            len: 1,
            engine: e.1.clone(),
        });
    }

    output
}

fn collect_scheme(scheme: Vec<Vec<Engine>>) -> Vec<Vec<SchemeNum>> {
    scheme
        .into_iter()
        .enumerate()
        .map(|(i, l)| collect_nums_from_scheme_line(l, i))
        .collect()
}

fn collect_nums_from_scheme_line(scheme_line: Vec<Engine>, index: usize) -> Vec<SchemeNum> {
    let mut iter = scheme_line.iter().enumerate();
    let mut output: Vec<SchemeNum> = vec![];

    while let Some(e) = iter.find(|&e| match e.1 {
        Engine::Digit(_) => true,
        _ => false,
    }) {
        output.push(get_number_from_scheme_line(
            &scheme_line[e.0..],
            (index, e.0),
        ));
        iter.find(|&e| match e.1 {
            Engine::Digit(_) => false,
            _ => true,
        });
    }

    output
}

fn get_number_from_scheme_line(slice: &[Engine], (x, y): (usize, usize)) -> SchemeNum {
    let mut iter = slice.iter();

    let mut acc = 0;
    let mut len = 0;
    while let Engine::Digit(n) = iter.next().unwrap_or(&Engine::Empty) {
        acc = acc * 10 + n;
        len += 1;
    }

    SchemeNum {
        x,
        y,
        len,
        engine: Engine::Digit(acc),
    }
}

fn get_num_from_scheme(
    head: (usize, usize, bool),
    tail: (usize, usize),
    scheme: &Vec<Vec<Engine>>,
) -> u32 {
    let mut total = 0;
    let mut place = 1;

    print!("{},{}:", head.0, head.1);

    for i in (head.1..(tail.1 + 1)).rev() {
        match scheme[head.0][i] {
            Engine::Digit(n) => total += n * place,
            _ => (),
        }
        place *= 10;
    }
    print!("{total}");
    total
}

fn map_engine_string_to_engine(line: String) -> Vec<Engine> {
    line.chars()
        .into_iter()
        .map(|c| match c {
            '.' => Engine::Empty,
            '*' => Engine::Part(Part::Gear),
            '&' | '#' | '+' | '/' | '=' | '-' | '\\' | '@' | '%' | '$' => Engine::Part(Part::Other),
            n => Engine::Digit(n.to_digit(10).unwrap()),
        })
        .collect()
}

fn day_two() {
    let stream = get_file_stream(String::from("cube_conundrum"));
    let result = stream
        .lines()
        .map(|l| parse_cube_game_day(l.unwrap()))
        .collect::<Vec<_>>();
    let real_result = result
        .into_iter()
        .fold(0, |acc, r| acc + game_is_impossible_id_total_for_day(r));
    print!("day two answer: {}\n", real_result);
}

struct CubeGame {
    red: u32,
    green: u32,
    blue: u32,
}

fn game_is_impossible_id_total_for_day(day: (usize, Vec<CubeGame>)) -> u32 {
    let mut max_cube = CubeGame {
        red: 0,
        green: 0,
        blue: 0,
    };
    for cb in day.1 {
        max_cube.red = cmp::max(max_cube.red, cb.red);
        max_cube.green = cmp::max(max_cube.green, cb.green);
        max_cube.blue = cmp::max(max_cube.blue, cb.blue);
    }
    max_cube.red * max_cube.green * max_cube.blue
}

fn parse_cube_game_day(line: String) -> (usize, Vec<CubeGame>) {
    let mut split = line.split(":");
    let index = split.next().unwrap();
    let day = split.next().unwrap();
    let index: usize = index.split(" ").collect::<Vec<&str>>()[1]
        .trim()
        .parse()
        .unwrap();

    let day: Vec<&str> = day.split(";").collect();
    let mut cube_games: Vec<CubeGame> = Vec::new();

    for game in day {
        let round: HashMap<&str, u32> = game
            .split(",")
            .map(|s| {
                let spl: Vec<&str> = s.split(" ").collect();
                (spl[2], spl[1].parse().unwrap())
            })
            .collect();

        let cube_game = CubeGame {
            red: *round.get(&"red").unwrap_or(&0),
            green: *round.get(&"green").unwrap_or(&0),
            blue: *round.get(&"blue").unwrap_or(&0),
        };

        cube_games.push(cube_game);
    }

    (index, cube_games)
}

fn day_one() {
    let stream = get_file_stream(String::from("calibration"));
    let result = stream
        .lines()
        .map(|l| select_only_digits(l.unwrap()))
        .fold(0, |acc, x| acc + get_first_and_last_digits(x));
    print!("day one answer: {result}\n");
}

fn select_only_digits(input: String) -> Vec<u32> {
    let numbers = HashMap::from([
        ("zero", 0),
        ("0", 0),
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ]);
    let mut number_indices: HashMap<usize, u32> = HashMap::new();

    for (word, number) in numbers {
        let matches: Vec<(usize, &str)> = input.match_indices(word).collect();
        for (index, _) in matches {
            number_indices.insert(index, number);
        }
    }

    let mut as_vec: Vec<(usize, u32)> = number_indices.into_iter().collect();
    as_vec.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    as_vec.into_iter().map(|(_, number)| number).collect()
}

fn get_file_stream(filename: String) -> BufReader<File> {
    let file = File::open(format!("files/{filename}")).unwrap();
    BufReader::new(file)
}

fn get_first_and_last_digits(input: Vec<u32>) -> u32 {
    input.first().unwrap() * 10 + input.last().unwrap()
}
