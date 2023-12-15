#![allow(dead_code)]

use rayon::prelude::*;

#[derive(Debug)]
struct Mapping {
    input_start: usize,
    output_start: usize,
    seq_len: usize,
}

#[derive(Debug)]
struct Puzzle {
    inputs: Vec<usize>,
    maps: Vec<Vec<Mapping>>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
            maps: Vec::new(),
        }
    }

    fn input_lines<'a, I>(&mut self, lines: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut lines = lines.into_iter();
        let seed_line = lines.next().unwrap();
        let (_label, numbers) = seed_line.split_once(": ").unwrap();
        self.inputs = numbers
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let _blank = lines.next();

        let mut current_mapping: Vec<Mapping> = Vec::new();
        for line in lines {
            if line == "" {
                self.maps.push(current_mapping);
                current_mapping = Vec::new();
            } else if line.chars().nth(0).unwrap().is_digit(10) {
                let numbers: Vec<usize> = line
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();
                current_mapping.push(Mapping {
                    input_start: numbers[1],
                    output_start: numbers[0],
                    seq_len: numbers[2],
                });
            }
        }
        self.maps.push(current_mapping);

        //dbg!(&self.inputs);
        //dbg!(&self.maps);
    }

    fn answer(&mut self) -> usize {
        let inputs = self
            .inputs
            .par_chunks_exact(2)
            .flat_map(|chunk| (chunk[0]..(chunk[0] + chunk[1])));
        inputs
            .map(|num| Self::apply_mappings(&self.maps, num))
            .min()
            .unwrap()
    }

    fn apply_mappings(maps: &Vec<Vec<Mapping>>, num: usize) -> usize {
        let mut o = num;
        for map in maps {
            for m in map {
                if (o >= m.input_start) && (o < m.input_start + m.seq_len) {
                    // println!("  {0} <= {n} < {1}",m.input_start,m.input_start+m.seq_len);
                    o = m.output_start + (o - m.input_start);
                    // println!("  {n} => {o}");
                    break;
                }
            }
        }
        o
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
    fn test_solving() {
        let mut puzzle = Puzzle::new();
        let answer = puzzle.solve(include_str!("../../test.txt"));
        assert_eq!(46, answer);
    }
}
