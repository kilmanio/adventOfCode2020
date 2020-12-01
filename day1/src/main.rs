use std::fs::File;
use std::io::{prelude::*, BufReader};
//use std::path::Path;

fn populate() -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        v.push(line.unwrap().parse::<u32>().unwrap());
    }

    v
}

fn resolve(numbers: Vec<u32>) {
    let goal = 2020;
    let mut result = 0;

    for number in &numbers {
        let target = goal - number;
        if numbers.contains(&target) {
            result = number * target;
            break;
        }
    }
    println!("{}", result);
}

fn main() {
    //let numbers = populate();
    let numbers: Vec<u32>;
    numbers = populate();
    //populate2(&mut numbers);
    resolve(numbers);
}

