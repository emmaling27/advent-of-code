use crate::Solution;

pub struct Day1 {
    pub input: String,
}

impl Solution for Day1 {
    fn part_1(&self) {
        let depths: Vec<i32> = self
            .input
            .split('\n')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let mut increases = 0;
        for i in 1..depths.len() {
            if depths[i] > depths[i - 1] {
                increases += 1;
            }
        }
        println!("{}", increases);
    }

    fn part_2(&self) {
        let depths: Vec<i32> = self
            .input
            .split('\n')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let mut increases = 0;
        for i in 3..depths.len() {
            if depths[i] > depths[i - 3] {
                increases += 1;
            }
        }
        println!("{}", increases);
    }
}
