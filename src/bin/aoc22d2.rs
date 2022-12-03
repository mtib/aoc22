use std::{fs, path::PathBuf};

fn main() {
    let root_dir = env!("CARGO_MANIFEST_DIR");
    let mut file_path = PathBuf::from(root_dir);
    file_path.push("./res/d2elf_strategy.txt");

    let strategy_txt: String =
        fs::read_to_string(file_path.to_str().expect("file path could not be parsed"))
            .expect("input could not be read");

    let rounds: Vec<game::Round> = strategy_txt
        .split("\n")
        .map(|row| row.try_into().expect("Faield to parse round"))
        .collect();

    let player_points: isize = rounds.iter().map(game::Round::score_for_player).sum();
    println!("#1 Player points: {}", player_points);

    // --- part 2 ---

    let rounds: Vec<game::Round> = strategy_txt
        .split("\n")
        .map(|row| {
            let row_strategy: game::RoundStrategy = row.try_into().expect("Faield to parse round");
            (&row_strategy).into()
        })
        .collect();

    let player_points: isize = rounds.iter().map(game::Round::score_for_player).sum();
    println!("#2 Player points: {}", player_points);
}

pub mod game {
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Action {
        Rock,
        Paper,
        Scissor,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Outcome {
        Win,
        Draw,
        Loss,
    }

    impl Action {
        fn cmp(&self, other: &Action) -> Outcome {
            if self == other {
                return Outcome::Draw;
            }
            match (self, other) {
                (&Action::Rock, &Action::Scissor)
                | (&Action::Paper, &Action::Rock)
                | (&Action::Scissor, &Action::Paper) => Outcome::Win,
                _ => Outcome::Loss,
            }
        }

        fn find_answer(&self, wanted_outcome: &Outcome) -> Self {
            for action in [Action::Rock, Action::Paper, Action::Scissor] {
                if &action.cmp(self) == wanted_outcome {
                    return action;
                }
            }
            unreachable!("Some action will always win Rock-Paper-Scissors");
        }
    }

    pub struct Round(Action, Action);

    impl TryFrom<&str> for Round {
        type Error = String;

        /// `value` in shape "A X"
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Ok(Round(
                match value.chars().nth(0) {
                    Some('A') => Action::Rock,
                    Some('B') => Action::Paper,
                    Some('C') => Action::Scissor,
                    _ => return Err(format!("cannot parse first action of \"{}\"", value)),
                },
                match value.chars().nth(2) {
                    Some('X') => Action::Rock,
                    Some('Y') => Action::Paper,
                    Some('Z') => Action::Scissor,
                    _ => return Err(format!("cannot parse second action of \"{}\"", value)),
                },
            ))
        }
    }

    impl From<&RoundStrategy> for Round {
        fn from(value: &RoundStrategy) -> Self {
            Round(value.0.clone(), value.0.find_answer(&value.1))
        }
    }

    impl Round {
        pub fn score_for_player(&self) -> isize {
            let play_bonus = match self {
                &Round(_, Action::Rock) => 1,
                &Round(_, Action::Paper) => 2,
                &Round(_, Action::Scissor) => 3,
            };
            let outcome_bonus = match self.1.cmp(&self.0) {
                Outcome::Win => 6,
                Outcome::Draw => 3,
                Outcome::Loss => 0,
            };
            play_bonus + outcome_bonus
        }
    }

    pub struct RoundStrategy(Action, Outcome);

    impl TryFrom<&str> for RoundStrategy {
        type Error = String;

        /// `value` in shape "A X"
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Ok(RoundStrategy(
                match value.chars().nth(0) {
                    Some('A') => Action::Rock,
                    Some('B') => Action::Paper,
                    Some('C') => Action::Scissor,
                    _ => return Err(format!("cannot parse action of \"{}\"", value)),
                },
                match value.chars().nth(2) {
                    Some('X') => Outcome::Loss,
                    Some('Y') => Outcome::Draw,
                    Some('Z') => Outcome::Win,
                    _ => return Err(format!("cannot parse outcome of \"{}\"", value)),
                },
            ))
        }
    }
}
