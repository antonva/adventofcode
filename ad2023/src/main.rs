mod day1;
mod day2;
mod util;

fn main() {
    let inputs = [
        "./src/day1_input",
        "./src/day2_input",
        //"./src/day3_input",
        //"./src/day4_input",
        //"./src/day5_input",
        //"./src/day6_input",
        //"./src/day7_input",
        //"./src/day8_input",
    ];

    for (i, input) in inputs.iter().enumerate() {
        let input_string = util::load_input(input);
        match i {
            0 => day1::exec(&input_string),
            1 => day2::exec(&input_string),
            _ => panic!("Day: {} not implemented yet!", i + 1),
        }
    }
}
