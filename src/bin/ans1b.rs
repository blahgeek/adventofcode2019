use std::io;

fn fuel_for_mass(mass: i64) -> i64 {
    i64::max(mass / 3 - 2, 0)
}

fn fuel_for_fuel(fuel_mass: i64) -> i64 {
    let additional_fuel_mass = fuel_for_mass(fuel_mass);
    if additional_fuel_mass > 0 {
        additional_fuel_mass + fuel_for_fuel(additional_fuel_mass)
    } else {
        additional_fuel_mass
    }
}

fn fuel_for_cargo(cargo_mass: i64) -> i64 {
    let fuel = fuel_for_mass(cargo_mass);
    fuel + fuel_for_fuel(fuel)
}

#[test]
fn fuel_for_cargo_test() {
    assert_eq!(fuel_for_cargo(14), 2);
    assert_eq!(fuel_for_cargo(1969), 966);
    assert_eq!(fuel_for_cargo(100756), 50346);
}

fn main() {
    let ans = io::stdin().lines()
        .filter_map(|line| line.ok().map(|line| line.trim().parse::<i64>().unwrap()))
        .map(fuel_for_cargo)
        .sum::<i64>();
    println!("{}", ans);
}
