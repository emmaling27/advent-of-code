use challenges::day_2::Day2;
use challenges::get_input;
use challenges::Solution;

fn main() {
    let day = Day2 {
        input: get_input("2"),
    };
    day.part_1();
    day.part_2();
}
