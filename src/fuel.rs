use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;
use std::error::Error;
use std::num::ParseIntError;

pub struct Module {
    mass: u32
}

pub fn load_modules(module_masses: &Path) -> Result<Vec<Module>, Box<dyn Error>> {
    let file = File::open(module_masses)?;
    let reader = BufReader::new(file);

    return Ok(reader.lines()
        // Fail if any line fails to read, passing back the io::Error
        .collect::<io::Result<Vec<String>>>()?.into_iter()
        .map(|s| s.parse::<u32>())
        // Fail if any line fails to parse as a u32, passing back the ParseIntError
        .collect::<Result<Vec<u32>, ParseIntError>>()?.into_iter()
        .map(|mass| Module{mass})
        .collect())
}

pub fn fuel_requirement(module: Module) -> u32 {
    let mass_div_three = module.mass / 3;
    return if mass_div_three < 2 { 0 } else { mass_div_three - 2 };
}

pub fn recursive_fuel_requirement(module: Module) -> u32 {
    let mass_div_three = module.mass / 3;
    return if mass_div_three < 2 { 0 } else {
        let fr = mass_div_three - 2;
        return fr + recursive_fuel_requirement(Module{mass: fr}) };
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

    #[test]
    fn test_recursive_fuel_requirement() {
        assert_eq!(recursive_fuel_requirement(Module{mass: 14}), 2);
        assert_eq!(recursive_fuel_requirement(Module{mass: 1969}), 966);
        assert_eq!(recursive_fuel_requirement(Module{mass: 100756}), 50346);
    }
}
