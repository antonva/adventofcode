use std::fmt;
use std::str::FromStr;

/// --- Day 5: Supply Stacks ---
///
/// The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.
///
/// The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.
///
/// The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
///
/// They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:
///
///     [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2
///
/// In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.
///
/// Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
///
/// [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
///
/// In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:
///
///         [Z]
///         [N]
///     [C] [D]
///     [M] [P]
///  1   2   3
///
/// Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:
///
///         [Z]
///         [N]
/// [M]     [D]
/// [C]     [P]
///  1   2   3
///
/// Finally, one crate is moved from stack 1 to stack 2:
///
///         [Z]
///         [N]
///         [D]
/// [C] [M] [P]
///  1   2   3
///
/// The Elves just need to know which crate will end up on top of each stack;
/// in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.
///
/// After the rearrangement procedure completes, what crate ends up on top of each stack?
///
/// --- Part Two ---
///
/// As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.
///
/// Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.
///
/// The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.
///
/// Again considering the example above, the crates begin in the same configuration:
///
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// Moving a single crate from stack 2 to stack 1 behaves the same as before:
///
/// [D]        
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:
///
///         [D]
///         [N]
///     [C] [Z]
///     [M] [P]
///  1   2   3
///
/// Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:
///
///         [D]
///         [N]
/// [C]     [Z]
/// [M]     [P]
///  1   2   3
///
/// Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:
///
///         [D]
///         [N]
///         [Z]
/// [M] [C] [P]
///  1   2   3
///
/// In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.
///
/// Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?

#[derive(Debug)]
struct InstructionNotFoundError;

impl fmt::Display for InstructionNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid instruction")
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Move {
        count: usize,
        from: usize,
        to: usize,
    },
}

impl FromStr for Instruction {
    type Err = InstructionNotFoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, arguments) = s
            .split_once(" ")
            .expect("Malformed input, could not split on space");
        match instruction {
            "move" => {
                let split_args: Vec<usize> = arguments
                    .split_whitespace()
                    .map(|arg| match arg.parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => 0,
                    })
                    .filter(|&x| x > 0)
                    .collect();

                if split_args.len() != 3 {
                    panic!("Invalid argument to instruction")
                }

                return Ok(Instruction::Move {
                    count: split_args[0],
                    from: split_args[1],
                    to: split_args[2],
                });
            }
            _ => return Err(InstructionNotFoundError),
        };
    }
}

pub fn exec(input: &str) {
    let res1 = testable_exec_part1(input);
    let res2 = testable_exec_part2(input);
    println!("Day 5\n\tpart 1:\t{res1}\n\tpart 2:\t{res2}");
}

fn testable_exec_part1(input: &str) -> String {
    let mut s = String::new();
    // Split stack initial state from instructions
    match input.split_once("\n\n") {
        Some((stack_description, instructions)) => {
            let mut stacks = chunk_description(stack_description);

            for il in instructions.lines() {
                match Instruction::from_str(il) {
                    Ok(instruction) => {
                        process_instruction(&instruction, &mut stacks);
                    }
                    Err(e) => panic!("{}", e),
                };
            }
            for stack in stacks {
                match stack.last() {
                    Some(l) => {
                        s.push(*l);
                    }
                    None => (),
                }
            }
        }
        None => panic!("Couldn't split description and instructions"),
    }
    s
}
fn testable_exec_part2(input: &str) -> String {
    let mut s = String::new();
    // Split stack initial state from instructions
    match input.split_once("\n\n") {
        Some((stack_description, instructions)) => {
            let mut stacks = chunk_description(stack_description);

            for il in instructions.lines() {
                match Instruction::from_str(il) {
                    Ok(instruction) => {
                        process_instruction_part2(&instruction, &mut stacks);
                    }
                    Err(e) => panic!("{}", e),
                };
            }
            for stack in stacks {
                match stack.last() {
                    Some(l) => {
                        s.push(*l);
                    }
                    None => (),
                }
            }
        }
        None => panic!("Couldn't split description and instructions"),
    }
    s
}

fn process_instruction(instruction: &Instruction, stacks: &mut Vec<Vec<char>>) {
    match instruction {
        Instruction::Move { count, from, to } => {
            for _ in 0..*count {
                let (from_stack, to_stack) = move_crate(
                    stacks.get(from - 1).unwrap().clone(),
                    stacks.get(to - 1).unwrap().clone(),
                );
                stacks[from - 1] = from_stack;
                stacks[to - 1] = to_stack
            }
        }
    }
}

fn process_instruction_part2(instruction: &Instruction, stacks: &mut Vec<Vec<char>>) {
    match instruction {
        Instruction::Move { count, from, to } => {
            let (from_stack, to_stack) = move_crates(
                *count,
                &mut stacks.get(from - 1).unwrap().clone(),
                &mut stacks.get(to - 1).unwrap().clone(),
            );
            stacks[from - 1] = from_stack;
            stacks[to - 1] = to_stack
        }
    }
}

fn move_crate(from: Vec<char>, to: Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut from_clone = from.clone();
    let mut to_clone = to.clone();
    if from_clone.is_empty() {
        return (from_clone.to_vec(), to_clone.to_vec());
    }
    match from_clone.pop() {
        Some(elf_crate) => to_clone.push(elf_crate),
        None => (),
    }
    (from_clone.to_vec(), to_clone.to_vec())
}

fn move_crates(count: usize, from: &mut Vec<char>, to: &mut Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut tmpstack: Vec<char> = vec![];
    for _ in 0..count {
        match from.pop() {
            Some(e) => {
                tmpstack.push(e.to_owned());
            }
            None => (),
        }
    }

    to.extend(
        tmpstack
            .iter()
            .rev()
            .map(|e| e.to_owned())
            .collect::<Vec<char>>(),
    );

    (from.to_vec(), to.to_vec())
}

fn chunk_description(desc: &str) -> Vec<Vec<char>> {
    let mut desc_iter = desc.lines().rev();
    let stacknames: Vec<usize> = desc_iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in stacknames {
        stacks.push(vec![]);
    }

    for line in desc_iter {
        let line_iter = line.chars().skip(1);
        for (i, c) in line_iter.step_by(4).enumerate() {
            if !c.is_whitespace() {
                stacks[i].push(c)
            }
        }
    }
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_works() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";
        assert_eq!(testable_exec_part1(input), "CMZ".to_string());
    }

    #[test]
    fn part_2_example_works() {
        let input = "...";
        assert_eq!(testable_exec_part2(input), "CMZ".to_string());
    }

    #[test]
    fn mv_crates_works() {
        let mut stack1: Vec<char> = vec!['A', 'B', 'C'];
        let mut stack2: Vec<char> = vec!['D', 'E', 'F'];

        (stack1, stack2) = move_crates(2, &mut stack1, &mut stack2);
        (stack1, stack2) = move_crates(1, &mut stack1, &mut stack2);
        println!("{stack1:?}");
        println!("{stack2:?}");

        let restack1: Vec<char> = vec![];
        let restack2: Vec<char> = vec!['D', 'E', 'F', 'B', 'C', 'A'];
        assert!(
            restack1.iter().zip(stack1.iter()).all(|(a, b)| a == b),
            "stack1 != restack1"
        );
        assert!(
            restack2.iter().zip(stack2.iter()).all(|(a, b)| a == b),
            "stack2 != restack2"
        )
    }

    #[test]
    fn parse_instruction_works() {
        let input = "move 3 from 1 to 2\n";
        let x = Instruction::from_str(input).unwrap();
        assert_eq!(
            x,
            Instruction::Move {
                count: 3,
                from: 1,
                to: 2
            }
        )
    }

    #[test]
    fn chunk_desc_works() {
        let input = "[C]     [H]\n[D] [N] [X]\n 1   2   3 ";
        let res = chunk_description(input);
        println!("{res:?}");
    }

    #[test]
    fn mv_crate_works() {
        let mut stack1: Vec<char> = vec!['A', 'B', 'C'];
        let mut stack2: Vec<char> = vec!['D', 'E', 'F'];

        (stack1, stack2) = move_crate(stack1, stack2);
        (stack1, stack2) = move_crate(stack1, stack2);

        let restack1: Vec<char> = vec!['A'];
        let restack2: Vec<char> = vec!['D', 'E', 'F', 'C', 'B'];
        assert!(
            restack1.iter().zip(stack1.iter()).all(|(a, b)| a == b),
            "stack1 != restack1"
        );
        assert!(
            restack2.iter().zip(stack2.iter()).all(|(a, b)| a == b),
            "stack2 != restack2"
        )
    }
}
