use std::io;

fn main() {
    let ans = io::stdin().lines()
        .filter_map(|line| line.ok().map(|line| line.trim().parse::<i32>().unwrap()))
        .map(|mass| mass / 3 - 2)
        .sum::<i32>();
    println!("{}", ans);
}
