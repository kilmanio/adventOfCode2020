use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Passline {
    lower: u8,
    upper: u8,
    character: u8,
    password: String,
}

fn read_line(line: String) -> Option<Passline> {
    let mut split = line.split(' ');
    let mut numbers = split.next()?.split('-');
    let ret = Passline { 
        lower: numbers.next()?.parse::<u8>().ok()?,
        upper: numbers.next()?.parse::<u8>().ok()?,
        character: split.next()?.as_bytes()[0], 
        password: split.next()?.to_string()
    };

    Some(ret)
}

fn is_valid(info: &Passline) -> bool {
    let count = info.password
        .as_bytes()
        .iter()
        .filter(|&&c| c == info.character)
        .count() as u8;

    if count < info.lower || count > info.upper {
        return false;
    }

    true
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let num = reader
        .lines()
        .filter_map(|x| read_line(x.unwrap()))
        .filter(|x| is_valid(x))
        .count();

    println!("Amount of valid passwords: {}", num);
}

