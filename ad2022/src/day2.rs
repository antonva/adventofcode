/// --- Day 2: Rock Paper Scissors ---
///
/// The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage,
/// a giant Rock Paper Scissors tournament is already in progress.
///
/// Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock,
/// Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
/// If both players choose the same shape, the round instead ends in a draw.
///
/// Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win.
/// "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--"
/// Suddenly, the Elf is called away to help with someone's tent.
///
/// The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors.
/// Winning every time would be suspicious, so the responses must have been carefully chosen.
///
/// The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round.
/// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
/// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
///
/// Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
///
/// For example, suppose you were given the following strategy guide:
///
/// A Y
/// B X
/// C Z
///
/// This strategy guide predicts and recommends the following:
///
///     In the first round, your opponent will choose Rock (A), and you should choose Paper (Y).
///     This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
///     In the second round, your opponent will choose Paper (B), and you should choose Rock (X).
///     This ends in a loss for you with a score of 1 (1 + 0).
///     The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
///
/// In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
///
/// What would your total score be if everything goes exactly according to your strategy guide?

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_str(input: &str) -> RPS {
        match input {
            "A" | "X" => return RPS::Rock,
            "B" | "Y" => return RPS::Paper,
            "C" | "Z" => return RPS::Scissors,
            &_ => todo!(),
        }
    }

    fn points(self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn compare(self, second_hand: &RPS) -> i8 {
        match self {
            RPS::Rock => match second_hand {
                RPS::Rock => 0,
                RPS::Paper => -1,
                RPS::Scissors => 1,
            },
            RPS::Paper => match second_hand {
                RPS::Rock => 1,
                RPS::Paper => 0,
                RPS::Scissors => -1,
            },
            RPS::Scissors => match second_hand {
                RPS::Rock => -1,
                RPS::Paper => 1,
                RPS::Scissors => 0,
            },
        }
    }
}

/// --- Part Two ---
///
/// The Elf finishes helping with the tent and sneaks back over to you.
/// "Anyway, the second column says how the round needs to end: X means you need to lose,
/// Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
///
/// The total score is still calculated in the same way,
/// but now you need to figure out what shape to choose so the round ends as indicated.
/// The example above now goes like this:
///
///     In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
///     In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
///     In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
///
/// Now that you're correctly decrypting the ultra top secret strategy guide,
/// you would get a total score of 12.
///
/// Following the Elf's instructions for the second column,
/// what would your total score be if everything goes exactly according to your strategy guide?

pub fn exec(input: &str) {
    // Part 1
    let playbook: _ = input
        .lines()
        .map(|line| {
            let x: Vec<RPS> = line
                .split_whitespace()
                .map(|hand| RPS::from_str(hand))
                .collect();
            x
        })
        .collect::<Vec<Vec<RPS>>>();

    let result: u32 = playbook
        .iter()
        .map(|play| {
            let h1 = play.get(0).unwrap();
            let h2 = play.get(1).unwrap();
            match h2.compare(h1) {
                -1 => 0 + h2.points(),
                0 => 3 + h2.points(),
                1 => 6 + h2.points(),
                _ => todo!(),
            }
        })
        .sum();
    println!("Day 2");
    println!("\tpart 1:{result}");

    // Part 2
    let actual_score: u32 = input
        .lines()
        .map(|line| {
            let s: Vec<&str> = line.split_whitespace().collect();
            let opponent_hand = RPS::from_str(s.get(0).unwrap());
            let play_result = s.get(1).unwrap();
            let score;
            match play_result {
                &"X" => score = follow_play(&opponent_hand, "lose"),
                &"Y" => score = follow_play(&opponent_hand, "draw"),
                &"Z" => score = follow_play(&opponent_hand, "win"),
                &_ => score = 0,
            }
            score
        })
        .sum();
    println!("\tpart 2: {actual_score}");
}

fn follow_play(opposing_hand: &RPS, condition: &str) -> u32 {
    match condition {
        "lose" => match opposing_hand {
            RPS::Rock => RPS::Scissors.points() + 0,
            RPS::Paper => RPS::Rock.points() + 0,
            RPS::Scissors => RPS::Paper.points() + 0,
        },
        "draw" => match opposing_hand {
            RPS::Rock => RPS::Rock.points() + 3,
            RPS::Paper => RPS::Paper.points() + 3,
            RPS::Scissors => RPS::Scissors.points() + 3,
        },
        "win" => match opposing_hand {
            RPS::Rock => RPS::Paper.points() + 6,
            RPS::Paper => RPS::Scissors.points() + 6,
            RPS::Scissors => RPS::Rock.points() + 6,
        },
        &_ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn follow_play_works_with_example() {
        assert_eq!(follow_play(&RPS::Rock, "draw"), 4);
        assert_eq!(follow_play(&RPS::Rock, "lose"), 3);
        assert_eq!(follow_play(&RPS::Rock, "win"), 8);
        assert_eq!(follow_play(&RPS::Paper, "draw"), 5);
        assert_eq!(follow_play(&RPS::Paper, "lose"), 1);
        assert_eq!(follow_play(&RPS::Paper, "win"), 9);
        assert_eq!(follow_play(&RPS::Scissors, "draw"), 6);
        assert_eq!(follow_play(&RPS::Scissors, "lose"), 2);
        assert_eq!(follow_play(&RPS::Scissors, "win"), 7);
    }
}
