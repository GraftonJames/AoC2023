use crate::get_file_stream;
use core::cmp;
use std::collections::HashMap;
use std::io::BufRead;
pub fn day_two() {
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
