use std::fmt;

pub fn exec(input: &str) {
    println!("Day 2");
    let res1 = testable_exec_part1(input);
    let res2 = testable_exec_part2(input);

    println!("Day 2\n\tpart 1: {res1}\n\tpart 2: {res2}");
}

fn testable_exec_part1(input: &str) -> usize {
   0
}
 
fn testable_exec_part2(input: &str) -> usize {
    0
}

#[test]
fn example_works() {
    let example = "";
    assert_eq!(testable_exec_part1(example), 0)
}
