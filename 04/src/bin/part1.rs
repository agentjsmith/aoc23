use std::{
    env,
    fs::File,
    io::Read, i32, collections::BTreeSet,
};

struct Puzzle {
    sum: i32,
}

impl Puzzle {
    fn new() -> Self {
        Self { sum: 0 }
    }

    fn input_lines<'a,I>(&mut self, lines: I)
        where I: IntoIterator<Item = &'a str>
    {
        for line in lines {
            let (label,contents) = line.split_once(": ").unwrap();
            let (mine,winners) = contents.split_once(" | ").unwrap();

            let mine: BTreeSet<i32> = mine.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
            let winners: BTreeSet<i32> = winners.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();

            let num_winners = winners.intersection(&mine).count();
            let score = if num_winners > 0 { 1 << (num_winners-1) } else { 0 };
            
            self.sum += score;

            println!("{}: {}",label,score)
        }
    }

    fn chug(&mut self) {}

    fn show_answer(&self) {
        println!("Sum is {}",self.sum);
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
