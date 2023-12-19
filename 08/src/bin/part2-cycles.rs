#![allow(dead_code)]

use num::integer::Integer;
use rayon::prelude::*;
use rstest::rstest;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
struct Puzzle {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            directions: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    fn input_lines<'a, I>(&mut self, lines: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut lines = lines.into_iter();

        let directions: &str = lines.next().unwrap();
        let _blank = lines.next();

        let mut parsed_directions: Vec<Direction> = directions
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("bad direction!"),
            })
            .collect();
        self.directions.append(&mut parsed_directions);

        for line in lines {
            let (here, nodes) = line.split_once(" = ").unwrap();
            let nodes: &str = &nodes[1..nodes.len() - 1];
            let (leftnode, rightnode) = nodes.split_once(", ").unwrap();
            self.nodes.insert(
                here.to_string(),
                (leftnode.to_string(), rightnode.to_string()),
            );
        }
    }

    fn find_cycle_length(&self, start: &str) -> usize {
        let mut node = start;

        for (count, dir) in self.directions.repeat(1_000_000).iter().enumerate() {
            if node.chars().nth_back(0).unwrap() == 'Z' {
                return count;
            }

            let (left, right) = self.nodes.get(node).unwrap();
            node = match dir {
                Direction::Left => left,
                Direction::Right => right,
            };
        }
        0
    }

    fn answer(&mut self) -> usize {
        let nodes: Vec<&String> = self
            .nodes
            .keys()
            .filter(|&k| k.chars().nth_back(0).unwrap() == 'A')
            .collect();

        println!("Found {} starting nodes", nodes.len());

        let cycles: Vec<usize> = nodes
            .par_iter()
            .map(|&n| self.find_cycle_length(n))
            .collect();

        cycles.into_iter().reduce(|x, y| x.lcm(&y)).unwrap()
    }

    fn solve(&mut self, input: &str) -> usize {
        self.input_lines(input.lines());
        self.answer()
    }
}

fn main() {
    let mut puzzle = Puzzle::new();
    let answer = puzzle.solve(include_str!("../../input.txt"));

    println!("The answer is {}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solving_1() {
        let mut puzzle = Puzzle::new();
        let answer = puzzle.solve(include_str!("../../test1.txt"));
        assert_eq!(2, answer);
    }

    #[test]
    fn test_solving_2() {
        let mut puzzle = Puzzle::new();
        let answer = puzzle.solve(include_str!("../../test2.txt"));
        assert_eq!(6, answer);
    }

    #[test]
    fn test_solving_3() {
        let mut puzzle = Puzzle::new();
        let answer = puzzle.solve(include_str!("../../test3.txt"));
        assert_eq!(6, answer);
    }
}
