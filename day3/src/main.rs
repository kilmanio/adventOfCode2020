use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let width = 31;
    let mut pos_x = 0;

    let num_trees = reader
        .lines()
        .skip(1)
        .filter(|l| {
            pos_x += 3;
            if pos_x >= width {pos_x -= width};
            l.as_ref().unwrap().as_bytes()[pos_x] == 35
        })
        .count();

    println!("Trees in day1: {}", num_trees);
}

