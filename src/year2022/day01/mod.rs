const INPUT: &str = include_str!("./input.txt");

pub fn solve() {
    let elves = INPUT.split("\n\n");

    let mut elf_totals: Vec<u32> = elves.map(|elf_values| {
            elf_values
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        }).collect();

    elf_totals.sort();
    let part1 = elf_totals.last().unwrap();
    let part2: u32 = elf_totals.iter().rev().take(3).sum();
    println!("Part 1: {:#?}", part1);
    println!("Part 2: {:#?}", part2);
}
