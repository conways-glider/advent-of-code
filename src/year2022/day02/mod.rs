const INPUT: &str = include_str!("./input.txt");

pub fn solve() {
    let part1: u32 = parse_part1(INPUT).iter().map(|(elf_choice, self_choice)| {
        self_choice.get_choice_points() + play_game(elf_choice, self_choice)
    }).sum();
    let part2: u32 = parse_part2(INPUT).iter().map(|(elf_choice, self_outcome)| {
        let self_choice = self_outcome.get_choice(elf_choice);
        self_choice.get_choice_points() + play_game(elf_choice, &self_choice)
    }).sum();
    println!("Part 1: {:#?}", part1);
    println!("Part 2: {:#?}", part2);
}

fn parse_part1(input: &str) -> Vec<(Game, Game)> {
    input
        .lines()
        .map(|line| {
            let head = line.chars().next().unwrap();
            let last = line.chars().last().unwrap();
            let elf_choice = match head {
                'A' => Game::Rock,
                'B' => Game::Paper,
                'C' => Game::Scissors,
                _ => unimplemented!(),
            };
            let self_choice = match last {
                'X' => Game::Rock,
                'Y' => Game::Paper,
                'Z' => Game::Scissors,
                _ => unimplemented!(),
            };
            (elf_choice, self_choice)
        })
        .collect()
}

fn parse_part2(input: &str) -> Vec<(Game, Outcome)> {
    input
        .lines()
        .map(|line| {
            let head = line.chars().next().unwrap();
            let last = line.chars().last().unwrap();
            let elf_choice = match head {
                'A' => Game::Rock,
                'B' => Game::Paper,
                'C' => Game::Scissors,
                _ => unimplemented!(),
            };
            let self_choice = match last {
                'X' => Outcome::Lose,
                'Y' => Outcome::Draw,
                'Z' => Outcome::Win,
                _ => unimplemented!(),
            };
            (elf_choice, self_choice)
        })
        .collect()
}

#[derive(PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn get_choice(self: &Outcome, others_choice: &Game) -> Game {
        match self {
            Outcome::Lose => match others_choice {
                Game::Rock => Game::Scissors,
                Game::Paper => Game::Rock,
                Game::Scissors => Game::Paper,
            },
            Outcome::Draw =>  match others_choice {
                Game::Rock => Game::Rock,
                Game::Paper => Game::Paper,
                Game::Scissors => Game::Scissors,
            },
            Outcome::Win =>  match others_choice {
                Game::Rock => Game::Paper,
                Game::Paper => Game::Scissors,
                Game::Scissors => Game::Rock,
            },
        }
    }
}

#[derive(PartialEq, Eq)]
enum Game {
    Rock,
    Paper,
    Scissors,
}

impl Game {
    fn get_choice_points(self: &Game) -> u32 {
        match self {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissors => 3,
        }
    }
}

fn play_game(elf_choice: &Game, self_choice: &Game) -> u32 {
    match (elf_choice, self_choice) {
        (x, y) if x == y => 3,
        (Game::Rock, Game::Paper) => 6,
        (Game::Paper, Game::Scissors) => 6,
        (Game::Scissors, Game::Rock) => 6,
        _ => 0,
    }
}
