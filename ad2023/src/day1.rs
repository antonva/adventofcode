use std::cmp::Ord;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn exec(input: &str) {
    let res1 = testable_exec_part1(input);
    let res2 = testable_exec_part2(input);

    println!("Day 1\n\tpart 1: {res1}\n\tpart 2: {res2}");
}

///--- Day 1: Trebuchet?! ---
///
///Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
///
///You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
///
///Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
///
///You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
///
///As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
///
///The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
///
///For example:
///
///1abc2
///pqr3stu8vwx
///a1b2c3d4e5f
///treb7uchet
///
///In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
///
///Consider your entire calibration document. What is the sum of all of the calibration values?
fn testable_exec_part1(input: &str) -> usize {
    sum_lines(input, true)
}

/// --- Part Two ---
///
/// Your calculation isn't quite right.
/// It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight,
/// and nine also count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
///
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
///
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
fn testable_exec_part2(input: &str) -> usize {
    sum_lines(input, false)
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct OffsetDigitPair {
    offset: usize,
    digit: String,
}

fn sum_lines(input: &str, only_num_digit: bool) -> usize {
    let line_results = input
        .lines()
        .map(|line| extract_first_and_last_digits(line, only_num_digit))
        .collect::<Vec<Option<(String, String)>>>();

    let sum = line_results
        .iter()
        .filter_map(|pos| match pos {
            Some((first, last)) => match format!("{first}{last}").parse::<usize>() {
                Ok(parsed_number) => return Some(parsed_number),
                Err(_) => panic!("Could not parse number from: {first} {last}"),
            },
            None => return None,
        })
        .sum::<usize>();
    sum
}

fn extract_first_and_last_digits(input: &str, only_num_digit: bool) -> Option<(String, String)> {
    let text_digits = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut first_heap = BinaryHeap::new();
    let mut last_heap = BinaryHeap::new();

    for (text_digit, num_digit) in text_digits {
        // Find text digits
        if !only_num_digit {
            input.match_indices(text_digit).for_each(|digit| {
                last_heap.push(OffsetDigitPair {
                    offset: digit.0,
                    digit: num_digit.to_string(),
                });
                first_heap.push(Reverse(OffsetDigitPair {
                    offset: digit.0,
                    digit: num_digit.to_string(),
                }));
            });
        }

        // Find numerical digits
        input.match_indices(num_digit).for_each(|digit| {
            last_heap.push(OffsetDigitPair {
                offset: digit.0,
                digit: num_digit.to_string(),
            });
            first_heap.push(Reverse(OffsetDigitPair {
                offset: digit.0,
                digit: num_digit.to_string(),
            }));
        });
    }

    if let Some(first_digit) = first_heap.peek() {
        if let Some(last_digit) = last_heap.peek() {
            return Some((first_digit.0.digit.clone(), last_digit.digit.clone()));
        }
    }
    None
}

#[test]
fn part1_test() {
    let example = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(testable_exec_part1(example), 142)
}

#[test]
fn part2_test() {
    let example = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
    assert_eq!(testable_exec_part2(example), 281)
}

#[test]
fn part2_test_samples_from_real_input() {
    let example = "
8eight1
    treb7uchet
    98126
    fourfourthreehnbhkmscqxdfksg64bvpppznkh
    8fivenvvtrlj
    six3zbhvrfhsevennine
    427nine6chnqrssxfour
    threevvxhvx38rktdqm3
    eightzgqzr3eight
    xgjjmnlvznf2nineltmsevennine6
    ninethree15seven
    37cjnsfbfkqpkxpdvgk8
    sgeightwo3
    9sbxg
    1spnthree59ninejjgjdlx
    six9six2gxmn
    twothreefdbl6five3zcqvcqxkcvdfkl4
    46six47seven27one
    four15
    7sixvmsrrzqnngonethree
    3fvsghvkqkbfivenine";
    assert_eq!(testable_exec_part2(example), 1263);
}
