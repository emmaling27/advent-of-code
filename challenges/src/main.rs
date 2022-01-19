use challenges::day_3::Day3;
use challenges::get_input;
use challenges::Solution;

fn main() {
    let day = Day3 {
        input: get_input("3"),
    };
    day.part_1();
    day.part_2();
}
