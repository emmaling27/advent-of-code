use std::fs;

fn day1part1() {
    let input = fs::read_to_string("inputs/1.txt")
        .expect("Something went wrong reading the file");
    let depths: Vec<i32> = input.split("\n").map(|s| s.parse::<i32>().unwrap()).collect();
    let mut increases = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i-1] {
            increases += 1;
        }
    }
    println!("{}", increases);
}

fn day1part2() {
    let input = fs::read_to_string("inputs/1.txt")
        .expect("Something went wrong reading the file");
    let depths: Vec<i32> = input.split("\n").map(|s| s.parse::<i32>().unwrap()).collect();
    let mut increases = 0;
    // let mut last_window = 0
    for i in 3..depths.len() {
        if depths[i] > depths[i-3] {
            increases += 1;
        }
    }
    println!("{}", increases);
}
fn main() {
    day1part2();
}
