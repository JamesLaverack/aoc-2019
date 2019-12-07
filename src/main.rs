use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;
use std::error::Error;
use std::num::ParseIntError;

fn main() {
    println!("Google Advent of Code 2019");

    let file_name = "day1";
    let file_path = Path::new(file_name);

    println!("Executing day one, part one on file {}", file_path.display());

    let total_fuel = match total_fuel_requirement(file_path) {
        Err(why) => panic!("Failed to calculate total fuel requirement: {}", why),
        Ok(total) => total,
    };
    println!("Total fuel requirement is {}", total_fuel)
}

fn total_fuel_requirement(module_masses: &Path) -> Result<u32, Box<dyn Error>> {
    let file = File::open(module_masses)?;
    let reader = BufReader::new(file);

    return Ok(reader.lines()
        // Fail if any line fails to read, passing back the io::Error
        .collect::<io::Result<Vec<String>>>()?.into_iter()
        .map(|s| s.parse::<u32>())
        // Fail if any line fails to parse as a u32, passing back the ParseIntError
        .collect::<Result<Vec<u32>, ParseIntError>>()?.into_iter()
        .map(|mass| Module{mass})
        .map(fuel_requirement)
        .sum())
}

struct Module {
    mass: u32
}

fn fuel_requirement(module: Module) -> u32 {
    let mass_div_three = module.mass / 3;
    return if mass_div_three < 2 { 0 } else { mass_div_three - 2 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_requirement() {
        assert_eq!(fuel_requirement(Module{mass: 12}), 2);
        assert_eq!(fuel_requirement(Module{mass: 14}), 2);
        assert_eq!(fuel_requirement(Module{mass: 1969}), 654);
        assert_eq!(fuel_requirement(Module{mass: 100756}), 33583);
        assert_eq!(fuel_requirement(Module{mass: 2}), 0);
        assert_eq!(fuel_requirement(Module{mass: 1}), 0);
        assert_eq!(fuel_requirement(Module{mass: 0}), 0);
    }
}
