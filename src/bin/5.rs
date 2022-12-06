use std::io;
use std::vec::Vec;
use std::collections::VecDeque;

use aoc2022::util::input_data_to_string;

#[derive(Clone,Debug)]
struct Stacks {
    stacks: Vec<VecDeque<char>>,
}

impl TryFrom<&str> for Stacks {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        // these are fixed with fields, letters are in columns (stack_num - 1) * 4 + 1
        let mut all_lines: Vec<&str> = s.lines().collect();
        assert!(" 1   2   3   4   5   6   7   8   9 ".contains(all_lines.pop().unwrap()));
        //println!("{}", all_lines.pop().unwrap());

        let mut stacks: Vec<VecDeque<char>> = Vec::new();
        stacks.resize(10, VecDeque::new());
        for line in all_lines {
            let c: Vec<char> = line.chars().collect();
            for i in 1..10 {
                let value = c[(i-1)*4+1];
                if value.is_uppercase() {
                    stacks[i].push_back(value);
                }
            }
        }
        
        Ok(Self{stacks})
    }
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

struct Program {
    instructions: Vec<Instruction>,
}

impl TryFrom<&str> for Instruction {
    type Error = ();
    
    fn try_from(s: &str) -> Result<Self, ()> {
        let v: Vec<&str> = s.split(' ').collect();
        Ok(Self{
            // not really checking validity
            count: v[1].parse().unwrap(),
            from: v[3].parse().unwrap(),
            to: v[5].parse().unwrap(),
        })
    }
}

impl TryFrom<&str> for Program {
    type Error = ();
    
    fn try_from(s: &str) -> Result<Self, ()> {
        Ok(Self{
            instructions: s.lines().map(|x| Instruction::try_from(x).unwrap()).collect(),
        })
    }
}

impl Program {
    fn execute(&self, stacks: & mut Stacks, whole_chunk: bool) {
        for i in &self.instructions {
            if whole_chunk {
                // the functionality of VecDeque makes this a bit clumsy
                let mut source = stacks.stacks[i.from].clone();
                let remain = source.split_off(i.count);
                let mut old_to = stacks.stacks[i.to].clone();
                stacks.stacks[i.to] = source;
                stacks.stacks[i.to].append(&mut old_to);
                stacks.stacks[i.from] = remain;
            } else {
                for _ in 0..i.count {
                    let val = stacks.stacks[i.from].pop_front().unwrap();
                    stacks.stacks[i.to].push_front(val);
                }
            }
        }
    }
}
fn main() -> io::Result<()> {

    let input = input_data_to_string("5.txt")?;
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let mut stacks_1 = Stacks::try_from(input_parts[0]).unwrap();
    let program = Program::try_from(input_parts[1]).unwrap();
    let mut stacks_2 = stacks_1.clone();
    program.execute(&mut stacks_1, false);
    let answer_1 = &(stacks_1.stacks[1..]).iter().map(|x| x.front().unwrap()).collect::<String>();
    program.execute(&mut stacks_2, true);
    let answer_2 = &(stacks_2.stacks[1..]).iter().map(|x| x.front().unwrap()).collect::<String>();
    println!("{answer_1} {answer_2}");
    Ok(())
}