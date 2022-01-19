use std::collections::HashMap;

use crate::Solution;

pub struct Day2 {
  pub input: String
}

impl Solution for Day2 {
  fn part_1(&self) {
    let commands = self.input.split('\n');
    let mut movement = HashMap::new();
    movement.insert("forward", 0);
    movement.insert("down", 0);
    movement.insert("up", 0);
    for command in commands {
      let components: Vec<&str> = command.split(' ').collect();
      let action  = components[0];
      let quantity = components[1].parse::<i32>().unwrap();
      movement.insert(action, movement.get(action).unwrap() + quantity);
    }
    println!("{}", movement.get("forward").unwrap() * (movement.get("down").unwrap() - movement.get("up").unwrap()));

  }

  fn part_2(&self) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    let commands = self.input.split('\n');
    for command in commands {
      let components: Vec<&str> = command.split(' ').collect();
      let action  = components[0];
      let quantity = components[1].parse::<i32>().unwrap();
      if action == "forward" {
        horizontal += quantity;
        depth += aim * quantity;
      } else if action == "down" {
        aim += quantity; 
      } else if action == "up" {
        aim -= quantity;
      }
    }
    println!("{}", horizontal * depth);
  }
}