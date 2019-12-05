fn main() {
    println!("Hello, world!");
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
