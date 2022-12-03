const INPUT: &str = include_str!("./input.txt");

pub fn solve() {
    let games = parse(INPUT);
    let part1: u32 = games.iter().map(|(elf_choice, self_choice)| {
        self_choice.get_choice_points() + play_game_part1(elf_choice, self_choice)
    }).sum();
    let part2 = "";
    println!("Part 1: {:#?}", part1);
    println!("Part 2: {:#?}", part2);
}

fn parse(input: &str) -> Vec<(Game, Game)> {
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

fn play_game_part1(elf_choice: &Game, self_choice: &Game) -> u32 {
    match (elf_choice, self_choice) {
        (x, y) if x == y => 3,
        (Game::Rock, Game::Paper) => 6,
        (Game::Paper, Game::Scissors) => 6,
        (Game::Scissors, Game::Rock) => 6,
        _ => 0,
    }
}
