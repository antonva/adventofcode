/// --- Day 8: Treetop Tree House ---
///
/// The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.
///
/// First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.
///
/// The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:
///
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
///
/// Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.
///
/// A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
///
/// All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:
///
///     The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
///     The top-middle 5 is visible from the top and right.
///     The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
///     The left-middle 5 is visible, but only from the right.
///     The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
///     The right-middle 3 is visible from the right.
///     In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
///
/// With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.
///
/// Consider your map; how many trees are visible from outside the grid?

pub fn exec(input: &str) {
    let res1 = testable_exec_part1(input);
    let res2 = testable_exec_part2(input);

    println!("Day 8\n\tpart 1:\t{res1}\n\tpart 2:\t{res2}");
}

fn testable_exec_part1(input: &str) -> usize {
    let grid = parse_input(input);
    let mut num_visible = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // edge cases are i == 0 and i == grid.len() - 1
            // top and bottom rows are always visible
            if i > 0 && i < (grid.len() - 1) {
                // edge cases are j == 0 and j == row.len() - 1
                // leftmost and rightmost sides are always visible
                if j > 0 && j < grid[i].len() - 1 {
                    let mut visible = false;
                    // do row sweep
                    // left
                    if !visible {
                        let mut local_vis = true;
                        for k in 0..j {
                            if grid[i][k] >= grid[i][j] {
                                local_vis = false;
                            }
                        }
                        visible = local_vis;
                    }
                    // right
                    if !visible {
                        let mut local_vis = true;
                        for k in (j + 1)..grid[i].len() {
                            if grid[i][k] >= grid[i][j] {
                                local_vis = false
                            }
                        }
                        visible = local_vis;
                    }
                    // do column sweep
                    // up
                    if !visible {
                        let mut local_vis = true;
                        for k in 0..grid.len() {
                            if grid[k][j] >= grid[i][j] {
                                local_vis = false;
                            }
                        }
                        visible = local_vis;
                    }
                    // down
                    if !visible {
                        let mut local_vis = true;
                        for k in (i + 1)..grid.len() {
                            if grid[k][j] >= grid[i][j] {
                                local_vis = false;
                            }
                        }
                        visible = local_vis;
                    }

                    if visible {
                        num_visible += 1;
                    }
                } else {
                    num_visible += 1;
                }
            } else {
                num_visible += 1;
            }
        }
    }
    num_visible
}

fn testable_exec_part2(input: &str) -> usize {
    0
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = vec![];
    for line in input.lines() {
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        res.push(row);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1_part_1_works() {
        let input = "30373\n25512\n65332\n33549\n35390";
        assert_eq!(testable_exec_part1(input), 21);
    }
    #[test]
    fn example2_part_1_works() {
        let input = "30373\n25512\n65332\n33549\n35390";
        assert_eq!(testable_exec_part1(input), 1);
    }
}
