use crate::get_file_stream;
use std::io::BufRead;

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

pub fn day_three() {
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

        let nums: Vec<(usize, usize, u32)> = nums
            .iter()
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
                let n2 = adj_nums.pop().unwrap();
                sum += n1 * n2;
            }
        }
    }
    print!("day three: {}\n", sum);
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
