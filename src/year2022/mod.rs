mod day01;
mod day02;

pub fn solve_all() {
    println!("Solving 2022");
    for day in 1..=2 {
        println!("Day {}", day);
        solve(day);
    }
}

pub fn solve(day: u8) {
    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        _ => unimplemented!(),
    }
}
