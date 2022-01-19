use std::fs;

pub mod day_1;
pub mod day_2;
pub mod day_3;

pub trait Solution {
    fn part_1(&self);
    fn part_2(&self);
}

pub fn get_input(filename: &str) -> String {
    fs::read_to_string(format!("inputs/{}.txt", filename))
        .expect("Something went wrong reading the file")
}
