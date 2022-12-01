/// --- Day 6: Tuning Trouble ---
///
/// The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the star fruit grove.
///
/// As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that it has many fancy features, but the most important one to set up right now is the communication system.
///
/// However, because he's heard you have significant experience dealing with signal-based systems, he convinced the other Elves that it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.
///
/// As if inspired by comedic timing, the device emits a few colorful sparks.
///
/// To be able to communicate with the Elves, the device needs to lock on to their signal. The signal is a series of seemingly-random characters that the device receives one at a time.
///
/// To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.
///
/// The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.
///
/// For example, suppose you receive the following datastream buffer:
///
/// mjqjpqmgbljsphdztnvjfqwrcgsmlb
///
/// After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj. Because j is repeated, this isn't a marker.
///
/// The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm, which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been processed.
///
/// Here are a few more examples:
///
///     bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
///     nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
///     nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
///     zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
///
/// How many characters need to be processed before the first start-of-packet marker is detected?
/// --- Part Two ---
///
/// Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for messages.
///
/// A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.
///
/// Here are the first positions of start-of-message markers for all of the above examples:
///
///     mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
///     bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
///     nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
///     nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
///     zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26
///
/// How many characters need to be processed before the first start-of-message marker is detected?

pub fn exec(input: &str) {
    let res1 = testable_exec_part1(input);
    let res2 = testable_exec_part2(input);

    println!("Day 6\n\tpart 1:\t{res1}\n\tpart 2:\t{res2}");
}

fn testable_exec_part1(input: &str) -> usize {
    find_start_of_markers(4, input)
}
fn testable_exec_part2(input: &str) -> usize {
    find_start_of_markers(14, input)
}

/// Generalized function for both problems.
/// Returns 0 on no marker found, not great.
fn find_start_of_markers(marker_len: usize, input: &str) -> usize {
    for i in 0..(input.len() - marker_len) {
        let input_iter = input.chars().skip(i);
        let x: Vec<char> = input_iter.take(marker_len).collect();
        // Keep original just in case
        let mut y = x.clone();
        y.sort();
        let mut dupes = false;
        for j in 1..y.len() {
            if y[j - 1] == y[j] {
                dupes = true;
                break;
            }
        }
        if dupes == false {
            return i + marker_len;
        }
    }
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1_part_1_works() {
        //     bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
        assert_eq!(testable_exec_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }
    #[test]
    fn example2_part_1_works() {
        //     nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
        assert_eq!(testable_exec_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }
    #[test]
    fn example3_part_1_works() {
        //     nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
        assert_eq!(testable_exec_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }
    #[test]
    fn example4_part_1_works() {
        //     zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
        assert_eq!(testable_exec_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn example1_part_2_works() {
        //mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
        assert_eq!(testable_exec_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }
    #[test]
    fn example2_part_2_works() {
        //bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
        assert_eq!(testable_exec_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }
    #[test]
    fn example3_part_2_works() {
        //nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
        assert_eq!(testable_exec_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }
    #[test]
    fn example4_part_2_works() {
        //nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
        assert_eq!(testable_exec_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }
    #[test]
    fn example5_part_2_works() {
        //zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26
        assert_eq!(testable_exec_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
