use std::io::prelude::*;

fn fuel_mass_for_module(module_mass: &i64) -> i64 {
    std::cmp::max(0, (*module_mass / 3) - 2)
}

fn fuel_mass_for_module_with_fuel_for_fuel(module_mass: &i64) -> i64 {
    if *module_mass <= 0 {
        return 0;
    }
    let module_fuel = fuel_mass_for_module(module_mass);
    module_fuel + fuel_mass_for_module_with_fuel_for_fuel(&module_fuel)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Provide one argument with the path to the input");
        return;
    }
    let file = std::fs::File::open(&args[1]).unwrap();
    let reader = std::io::BufReader::new(file);
    let module_masses: Vec<i64> = reader
        .lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .collect();
    let fuel_masses = module_masses.iter().map(fuel_mass_for_module);
    let proper_fuel_masses = module_masses
        .iter()
        .map(fuel_mass_for_module_with_fuel_for_fuel);
    let total_fuel: i64 = fuel_masses.sum();
    let proper_total_fuel: i64 = proper_fuel_masses.sum();

    println!("total fuel without fuel-for-fuel: {}", total_fuel);
    println!("total fuel with fuel-for-fuel: {}", proper_total_fuel);
}
