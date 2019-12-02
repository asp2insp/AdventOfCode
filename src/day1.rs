use std::collections::HashSet;

fn mass_to_fuel(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

pub fn part1(input: String) -> String {
    format!(
        "{}",
        input
            .lines()
            .flat_map(str::parse::<usize>)
            .map(mass_to_fuel)
            .sum::<usize>()
    )
}

fn module_fuel_rocket(mass: usize) -> usize {
    let mut fuel_mass = mass_to_fuel(mass);
    let mut mass_remaining = fuel_mass;
    while mass_remaining != 0 {
        mass_remaining = mass_to_fuel(mass_remaining);
        fuel_mass += mass_remaining;
    }
    fuel_mass
}

pub fn part2(input: String) -> String {
    let mut fuel_mass = input
        .lines()
        .flat_map(str::parse::<usize>)
        .map(module_fuel_rocket)
        .sum::<usize>();
    format!("{}", fuel_mass)
}
