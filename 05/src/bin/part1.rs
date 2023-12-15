#![allow(dead_code)]
use std::{
    env,
    fs::File,
    io::{Read, Write}, i32, collections::{BTreeSet, BTreeMap},
};

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
    outputs: Vec<usize>,
}

impl Puzzle {
    fn new() -> Self {
        Self { inputs: Vec::new(), maps: Vec::new(), outputs: Vec::new() }
    }

    fn input_lines<'a,I>(&mut self, lines: I)
        where I: IntoIterator<Item = &'a str>
    {
        let mut lines = lines.into_iter();
        let seed_line = lines.next().unwrap();
        let (_label,numbers) = seed_line.split_once(": ").unwrap();
        self.inputs=numbers.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();

        let _blank = lines.next();
        
        let mut current_mapping: Vec<Mapping> = Vec::new();
        while let Some(line) = lines.next() {
            if line == "" {
                self.maps.push(current_mapping);
                current_mapping = Vec::new();
            } else if line.chars().nth(0).unwrap().is_digit(10) {
                let numbers: Vec<usize> = line.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
                current_mapping.push(Mapping {input_start: numbers[1], output_start: numbers[0], seq_len: numbers[2]});
            }
        }
        self.maps.push(current_mapping);

        //dbg!(&self.inputs);
        //dbg!(&self.maps);
    }

    fn chug(&mut self) {
        let mut numbers = self.inputs.clone();
        dbg!(&numbers);
        for m in &self.maps {
            numbers = Self::apply_mappings(m,numbers);
        }
        self.outputs = numbers;
    }

    fn apply_mappings(map: &Vec<Mapping>, nums: Vec<usize>) -> Vec<usize> {
        let mut out = Vec::new();

        for n in nums {
            let mut o = n;
            for m in map {
                if (n >= m.input_start) && (n < m.input_start+m.seq_len) {
                    // println!("  {0} <= {n} < {1}",m.input_start,m.input_start+m.seq_len);
                    o = m.output_start + (n - m.input_start);   
                    // println!("  {n} => {o}");
                    break;
                }
            }
            out.push(o);
        }
        dbg!(out)
    }
    
    fn show_answer(&self) {
        println!("Nearest location is {}",self.outputs.iter().min().unwrap());
    }   
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let mut fh = File::open(input_file).expect("Could not open the input file");

    let mut contents = String::new();
    fh.read_to_string(&mut contents)
        .expect("Could not read the input file");

    let lines = contents.lines();
    let mut puzzle = Puzzle::new();
    puzzle.input_lines(lines);

    puzzle.chug();

    puzzle.show_answer();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_whatever() {
       assert!(true); 
    }
}
