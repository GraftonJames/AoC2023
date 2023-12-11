use crate::get_file_stream;
use std::io::BufRead;

pub fn day_four() {
    let stream = get_file_stream(String::from("scratchcards"));
    let mut card_stack: Vec<(usize, Vec<u16>, Vec<u16>)> = stream
        .lines()
        .map(|l| parse_line_to_card(l.unwrap()))
        .collect();

    for card_index in 0..card_stack.len() {
        let count = count_card_points(&card_stack[card_index]);
        for i in 1..=count {
            card_stack[card_index + i].0 += card_stack[card_index].0;
        }
    }

    let sum = card_stack.iter().fold(0, |acc, n| acc + n.0);

    print!("day four: {}\n", sum);
}

fn parse_line_to_card(line: String) -> (usize, Vec<u16>, Vec<u16>) {
    let mut line = line.split(":");
    line.next();
    let mut line = line.next().unwrap().split("|");
    let winning_cards = line.next().unwrap().split(" ");
    let my_cards = line.next().unwrap().split(" ");

    let winning_cards: Vec<u16> = winning_cards.filter_map(|n| n.parse().ok()).collect();
    let my_cards: Vec<u16> = my_cards.filter_map(|n| n.parse().ok()).collect();

    (1, winning_cards, my_cards)
}

fn count_card_points(
    (_card_count, winning_cards, my_cards): &(usize, Vec<u16>, Vec<u16>),
) -> usize {
    winning_cards.into_iter().fold(
        0,
        |acc, wc| {
            if my_cards.contains(&wc) {
                acc + 1
            } else {
                acc
            }
        },
    )
}
