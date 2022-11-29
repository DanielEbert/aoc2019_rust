mod util;

fn main() {
    let lines = util::read_lines("./inp.txt");

    let fuel_req: i32 = lines
        .map(|line| module_fuel(line.parse::<i32>().unwrap()))
        .sum();

    println!("Fuel Req: {}", fuel_req);

    println!("Test: {}", module_fuel(100756));
}

fn module_fuel(mass: i32) -> i32 {
    let mut fuel_req = mass_to_fuel(mass);
    let mut extra_fuel = mass_to_fuel(fuel_req);
    while extra_fuel > 0 {
        fuel_req += extra_fuel;
        extra_fuel = mass_to_fuel(extra_fuel);
    }
    fuel_req
}

fn mass_to_fuel(mass: i32) -> i32 {
    std::cmp::max(mass / 3 - 2, 0)
}
