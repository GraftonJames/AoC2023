use crate::get_file_stream;
use core::iter::zip;
use std::io::BufRead;
use std::io::Lines;

#[derive(Clone, Copy)]
struct MapPart {
    destination: u128,
    source: u128,
    range: u128,
}

#[derive(PartialEq, PartialOrd)]
struct SeedRange {
    start: u128,
    length: u128,
}

pub fn day_five() {
    let stream = get_file_stream(String::from("seedmaps"));
    let mut stream = stream.lines();

    let seeds: Vec<u128> = stream
        .next()
        .unwrap()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse().ok())
        .collect(); // What the fuck

    let seeds: Vec<SeedRange> = zip(
        seeds
            .iter()
            .enumerate()
            .filter(|(i, _)| *i % 2 == 0)
            .map(|(_, seed)| *seed)
            .collect::<Vec<u128>>(),
        seeds
            .iter()
            .enumerate()
            .filter(|(i, _)| *i % 2 == 1)
            .map(|(_, seed)| *seed)
            .collect::<Vec<u128>>(),
    )
    .map(|(start, length)| SeedRange { start, length })
    .collect();

    let maps: Vec<Vec<MapPart>> = get_maps(stream);

    let locations: Vec<SeedRange> = seeds
        .into_iter()
        .map(|s| get_location(s, maps))
        .flatten()
        .collect();

    print!(
        "day five answer: {}\n",
        locations.iter().map(|s| s.start).min().unwrap()
    );
}

fn get_location(source: SeedRange, maps: Vec<Vec<MapPart>>) -> Vec<SeedRange> {
    let map: Vec<MapPart> = maps.pop().unwrap();
    let chunked_seed_ranges: Vec<SeedRange> = vec![];
    let break_points: Vec<u128> = vec![];
    for part in map {
        break_points.push(part.source);
        break_points.push(part.source + part.range);
    }
    for bp in break_points {
        if bp >= source.start && bp < source.start + source.length {
            chunked_seed_ranges.push(SeedRange {
                start: source.start,
                length: bp - source.start,
            });
            source.start = bp
        } else if bp >= source.start + source.length {
            chunked_seed_ranges.push(SeedRange {
                start: source.start,
                length: (source.start + source.length) - bp,
            })
        }
    }
    let delegated_maps = maps.clone();
    chunked_seed_ranges
        .iter()
        .map(|sr| map_seed_range(*sr, &map))
        .map(|sr| get_location(sr, delegated_maps))
        .flatten()
        .collect()
}

fn map_seed_range(seed_range: SeedRange, map: &Vec<MapPart>) -> SeedRange {
    let mp = map.iter().find(|mp| mp.source <= seed_range.start && mp.source + mp.range >= seed_range.start + seed_range.length).unwrap();

    SeedRange {
        start: mp.destination + (seed_range.start - mp.source),
        length: seed_range.length
    }
    
}

fn get_maps<B: BufRead>(mut stream: Lines<B>) -> Vec<Vec<MapPart>> {
    let mut maps = vec![];
    stream.next();
    loop {
        let line = stream.next();
        if let Some(s) = line {
            if s.unwrap().contains(":") {
                let mut map = add_map(&mut stream);
                map.sort_by(|a, b| a.source.partial_cmp(&b.source).unwrap());
                maps.push(map);
            } else {
                return maps;
            }
        } else if let None = line {
            return maps;
        }
    }
}

fn add_map<B: BufRead>(stream: &mut Lines<B>) -> Vec<MapPart> {
    let mut map: Vec<MapPart> = vec![];
    loop {
        let line = stream.next();
        match line {
            Some(Err(_)) | None => return map,
            Some(Ok(ref s)) => {
                if s.as_str() == "" {
                    return map;
                } else {
                    let line: Vec<u128> = s.split(" ").filter_map(|n| n.parse().ok()).collect();
                    map.push(MapPart {
                        destination: line[0],
                        source: line[1],
                        range: line[2],
                    });
                }
            }
        }
    }
}
