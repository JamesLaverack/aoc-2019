fn main() {
    println!("Hello, world!");
}

struct Module {
    mass: u32
}

fn fuel_requirement(module: Module) -> u32 {
    return 0;
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
    }
}