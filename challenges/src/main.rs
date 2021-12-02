use challenges::day_1::Day1;
use challenges::get_input;
use challenges::Solution;

fn main() {
    let day = Day1 {
        input: get_input("1"),
    };
    day.part_1()
}
