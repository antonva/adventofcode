mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod util;

fn main() {
    let inputs = [
        "./src/day1_input",
        "./src/day2_input",
        "./src/day3_input",
        "./src/day4_input",
        "./src/day5_input",
        "./src/day6_input",
        "./src/day7_input",
        "./src/day8_input",
    ];

    for (i, input) in inputs.iter().enumerate() {
        let input_string = util::load_input(input);
        match i {
            0 => day1::exec(&input_string),
            1 => day2::exec(&input_string),
            2 => day3::exec(&input_string),
            3 => day4::exec(&input_string),
            4 => day5::exec(&input_string),
            5 => day6::exec(&input_string),
            6 => (), //day7::exec(&input_string),
            7 => day8::exec(&input_string),
            _ => panic!("Day: {} not implemented yet!", i + 1),
        }
    }
}
