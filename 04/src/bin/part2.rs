use std::{
    env,
    fs::File,
    io::Read, i32, collections::BTreeSet,
};

struct Puzzle {
    sum: i32,
    ticket_winners: Vec<i32>,
    ticket_copies: Vec<i32>,
}

impl Puzzle {
    fn new() -> Self {
        Self { sum: 0, ticket_winners: Vec::new(), ticket_copies: Vec::new() }
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
            self.ticket_winners.push(num_winners.try_into().unwrap());
            self.ticket_copies.push(1);
            
            println!("{}: {}",label,num_winners);
        }
    }

    fn chug(&mut self) {
        for i in 0..self.ticket_copies.len() {
            let winners = self.ticket_winners[i];
            if winners > 0 {
                for j in i+1..=winners as usize + i {
                    self.ticket_copies[j] += self.ticket_copies[i];
                }
            }
        }

        self.sum = self.ticket_copies.iter().sum();
    }

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
