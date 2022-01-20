use num::pow;

use crate::Solution;

pub struct Day3 {
  pub input: String
}

impl Solution for Day3 {
  fn part_1(&self) {
    let numbers = self.input.split('\n');
    let length: usize = numbers.clone().last().unwrap().len();
    let mut total = 0;
    let mut counts = vec![0; length];
    for n in numbers {
      for i in 0..length {
        counts[i] += n.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap();
      }
      total += 1;
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..length {
      let x = counts[i];
      // println!("{}",x);
      gamma += (x * 2 / total) * (pow(2, length-1-i));
      epsilon += (1 - (x * 2 / total)) * (pow(2,length-1-i))
    }
    // println!("{:?}", counts);
    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
  }

  fn part_2(&self) {

  }
}