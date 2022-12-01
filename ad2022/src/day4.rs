/// --- Day 4: Camp Cleanup ---
///
/// Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.
///
/// However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).
///
/// For example, consider the following list of section assignment pairs:
///
/// 2-4,6-8
/// 2-3,4-5
/// 5-7,7-9
/// 2-8,3-7
/// 6-6,4-6
/// 2-6,4-8
///
/// For the first few pairs, this list means:
///
///     Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
///     The Elves in the second pair were each assigned two sections.
///     The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
///
/// This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:
///
/// .234.....  2-4
/// .....678.  6-8
///
/// .23......  2-3
/// ...45....  4-5
///
/// ....567..  5-7
/// ......789  7-9
///
/// .2345678.  2-8
/// ..34567..  3-7
///
/// .....6...  6-6
/// ...456...  4-6
///
/// .23456...  2-6
/// ...45678.  4-8
///
/// Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.
///
/// In how many assignment pairs does one range fully contain the other?

pub fn exec(input: &str) {
    let res1 = testable_exec_1(&input);
    let res2 = testable_exec_2(&input);

    println!("Day 4\n\tpart 1:\t{res1}\n\tpart 2:\t{res2}");
}

fn testable_exec_1(input: &str) -> u32 {
    let x: u32 = input
        .lines()
        .map(|line| {
            // Safety: We know we'll have a clean input, unwrapping the
            // option is fine.
            let (elf1, elf2) = line.split_once(",").unwrap();
            let (e1_lo, e1_hi) = elf1.split_once("-").unwrap();
            let (e2_lo, e2_hi) = elf2.split_once("-").unwrap();
            let elf1_lower: u32 = e1_lo.parse::<u32>().unwrap();
            let elf1_higher: u32 = e1_hi.parse::<u32>().unwrap();
            let elf2_lower: u32 = e2_lo.parse::<u32>().unwrap();
            let elf2_higher: u32 = e2_hi.parse::<u32>().unwrap();

            let mut sum = 0;
            if elf1_higher > elf2_higher {
                if elf1_lower <= elf2_lower {
                    sum = 1;
                }
            }

            if elf1_higher <= elf2_higher {
                if elf1_lower >= elf2_lower {
                    sum = 1;
                }

                if elf1_lower < elf2_lower && elf2_higher == elf1_higher {
                    sum = 1;
                }
                // Corner case
                if elf2_higher == elf2_lower && elf2_higher == elf1_higher {
                    sum = 1;
                }
            }
            sum
        })
        .sum();

    x
}

fn testable_exec_2(input: &str) -> u32 {
    let x: u32 = input
        .lines()
        .map(|line| {
            // Safety: We know we'll have a clean input, unwrapping the
            // option is fine.
            let (elf1, elf2) = line.split_once(",").unwrap();
            let (e1_lo, e1_hi) = elf1.split_once("-").unwrap();
            let (e2_lo, e2_hi) = elf2.split_once("-").unwrap();
            let elf1_lower: u32 = e1_lo.parse::<u32>().unwrap();
            let elf1_higher: u32 = e1_hi.parse::<u32>().unwrap();
            let elf2_lower: u32 = e2_lo.parse::<u32>().unwrap();
            let elf2_higher: u32 = e2_hi.parse::<u32>().unwrap();

            let mut sum = 0;

            if elf1_higher > elf2_higher {
                if elf1_lower <= elf2_higher {
                    sum = 1;
                }
            }

            if elf1_higher < elf2_higher {
                if elf2_lower <= elf1_higher {
                    sum = 1;
                }
            }

            if elf1_higher == elf2_higher {
                // All variations on this arrangement end in an overlap
                sum = 1;
            }

            sum
        })
        .sum();

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_works() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n4-6,6-6";
        assert_eq!(testable_exec_1(input), 3)
    }
    #[test]
    fn part1_new_example_works() {
        let input = "1-10,2-9\n2-9,1-10\n2-2,2-3\n2-3,2-2\n1-5,5-5\n5-5,1-5\n1-4,2-4\n2-4,1-4";
        assert_eq!(testable_exec_1(input), 8)
    }

    #[test]
    fn part2_example_works() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n4-6,6-6";
        assert_eq!(testable_exec_2(input), 5)
    }
}
