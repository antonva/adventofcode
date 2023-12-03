use std::{fmt, str::FromStr};

pub fn exec(input: &str) {
    let res1 = testable_exec_part1(input);
    let res2 = testable_exec_part2(input);

    println!("Day 2\n\tpart 1: {res1}\n\tpart 2: {res2}");
}
/// --- Day 2: Cube Conundrum ---
///
/// You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky.
/// You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.
///
/// The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow.
/// He'll be happy to explain the situation, but it's a bit of a walk, so you have some time.
/// They don't get many visitors up here; would you like to play a game in the meantime?
///
/// As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue.
/// Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.
///
/// To get information, once a bag has been loaded with cubes, the Elf will reach into the bag,
/// grab a handful of random cubes, show them to you, and then put them back in the bag.
/// He'll do this a few times per game.
///
/// You play several games and record the information from each game (your puzzle input).
/// Each game is listed with its ID number (like the 11 in Game 11: ...)
/// followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
///
/// For example, the record of a few games might look like this:
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
///
/// In game 1, three sets of cubes are revealed from the bag (and then put back again).
/// The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.
///
/// The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
///
/// In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration.
/// However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been
/// impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.
///
/// Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes.
/// What is the sum of the IDs of those games?
fn testable_exec_part1(input: &str) -> usize {
    let (red, green, blue) = (12, 13, 14);
    let sum = input
        .lines()
        .map(|line| match Game::from_str(line) {
            Ok(game) => game,
            Err(e) => panic!("{e:?}"),
        })
        .filter(|game| game.possible(red, green, blue))
        .map(|game| game.id)
        .sum::<usize>();
    sum
}

/// --- Part Two ---
///
/// The Elf says they've stopped producing snow because they aren't getting any water!
/// He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself.
/// It's just up ahead!
///
/// As you continue your walk, the Elf poses a second question:
/// in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?
///
/// Again consider the example games from earlier:
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
///
///     In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
///     Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
///     Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
///     Game 4 required at least 14 red, 3 green, and 15 blue cubes.
///     Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
///
/// The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
/// The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively.
/// Adding up these five powers produces the sum 2286.
///
/// For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
fn testable_exec_part2(input: &str) -> usize {
    let sum = input
        .lines()
        .map(|line| match Game::from_str(line) {
            Ok(game) => game.power(),
            Err(e) => panic!("{e:?}"),
        })
        .sum::<usize>();
    sum
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    cubesets: Vec<CubeSet>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<&str> = s.split(":").collect();
        if line.len() != 2 {
            return Err(ParseGameError);
        }
        let id = line[0]
            .replace("Game ", "")
            .parse::<usize>()
            .map_err(|_| ParseGameError)?;
        let cubesets: Vec<CubeSet> = line[1]
            .split(";")
            .map(|input| CubeSet::from_str(input).expect("Could not parse cube set from string"))
            .collect();
        Ok(Game::new(id, cubesets))
    }
}

impl Game {
    fn new(id: usize, cubesets: Vec<CubeSet>) -> Self {
        Self { id, cubesets }
    }

    fn possible(&self, red: usize, green: usize, blue: usize) -> bool {
        let max = self.max_colors();
        max.red <= red && max.green <= green && max.blue <= blue
    }

    fn max_colors(&self) -> CubeSet {
        let mut cubeset = CubeSet::new(0, 0, 0);
        self.cubesets.iter().for_each(|set| {
            cubeset.red = cubeset.red.max(set.red);
            cubeset.green = cubeset.green.max(set.green);
            cubeset.blue = cubeset.blue.max(set.blue);
        });
        cubeset
    }

    fn power(&self) -> usize {
        let max = self.max_colors();
        max.red * max.green * max.blue
    }
}

#[derive(Debug, PartialEq)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseCubeSetError {
    RedFail,
    GreenFail,
    BlueFail,
    SplitFail,
    NonColorFail,
}

impl FromStr for CubeSet {
    type Err = ParseCubeSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let comma_sep: Vec<&str> = s.split(",").collect();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for c in comma_sep {
            match c.trim().split_once(" ") {
                Some((number, color)) => match color {
                    "red" => {
                        r = number
                            .parse::<usize>()
                            .map_err(|_| ParseCubeSetError::RedFail)?
                    }
                    "green" => {
                        g = number
                            .parse::<usize>()
                            .map_err(|_| ParseCubeSetError::GreenFail)?
                    }
                    "blue" => {
                        b = number
                            .parse::<usize>()
                            .map_err(|_| ParseCubeSetError::BlueFail)?
                    }
                    _ => return Err(ParseCubeSetError::NonColorFail),
                },
                None => return Err(ParseCubeSetError::SplitFail),
            }
        }
        Ok(CubeSet::new(r, g, b))
    }
}

impl CubeSet {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }
}

#[test]
fn cubeset_from_str_works() {
    let example = "3 blue, 4 red, 1 green";
    assert_eq!(
        CubeSet {
            red: 4,
            green: 1,
            blue: 3
        },
        CubeSet::from_str(example).unwrap()
    );
}
#[test]
fn game_from_str_works() {
    let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    assert_eq!(
        Game {
            id: 1,
            cubesets: vec![
                CubeSet {
                    red: 4,
                    blue: 3,
                    green: 0
                },
                CubeSet {
                    red: 1,
                    green: 2,
                    blue: 6
                },
                CubeSet {
                    red: 0,
                    green: 2,
                    blue: 0
                }
            ]
        },
        Game::from_str(example).unwrap()
    )
}

#[test]
fn game_possibility_works() {
    let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let game = Game::from_str(example).unwrap();
    assert!(game.possible(10, 10, 10));
    assert!(!game.possible(1, 1, 1));
}

#[test]
fn game_max_color_works() {
    let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let game = Game::from_str(example).unwrap();
    assert_eq!(
        game.max_colors(),
        CubeSet {
            red: 4,
            green: 2,
            blue: 6
        }
    )
}
#[test]
fn part_1_example_works() {
    let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(testable_exec_part1(example), 8)
}

#[test]
fn part_2_example_works() {
    let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(testable_exec_part2(example), 2286)
}
