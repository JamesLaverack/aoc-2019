use std::path::Path;
use std::fs;
use std::error::Error;

pub fn load(code_file: &Path) -> Result<Vec<u32>, Box<dyn Error>> {
    let code = fs::read_to_string(code_file)?
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, std::num::ParseIntError>>()?;
    return Ok(code)
}

pub fn run(code :&mut Vec<u32>) -> u32 {
    let mut program_counter :usize = 0;

    loop {
        match step(code, program_counter) {
            Ok(_) => (),
            Err(exit_code) => return exit_code,
        }
        program_counter += 4
    }
}

fn step(code :&mut Vec<u32>, program_counter :usize) -> Result<(), u32> {
    let opcode = code[program_counter];
    let f :fn(u32, u32) -> u32 = match opcode {
        1 => |a, b| a + b,
        2 => |a, b| a * b,
        error_code => return Err(error_code)
    };

    let input_one_pos = code[program_counter + 1] as usize;
    let input_two_pos = code[program_counter + 2] as usize;
    let output_pos = code[program_counter + 3] as usize;
    code[output_pos] = f(code[input_one_pos], code[input_two_pos]);
    return Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worked_example() {
        let mut code = vec!(1,9,10,3,2,3,11,0,99,30,40,50);
        let exit_code = run(&mut code);
        assert_eq!(exit_code, 99);
        assert_eq!(code, vec!(3500,9,10,70,2,3,11,0,99,30,40,50));
    }

    #[test]
    fn test_add() {
        let mut code = vec!(1,0,0,0,99);
        let exit_code = run(&mut code);
        assert_eq!(exit_code, 99);
        assert_eq!(code, vec!(2,0,0,0,99));
    }

    #[test]
    fn test_mul() {
        let mut code = vec!(2,3,0,3,99);
        let exit_code = run(&mut code);
        assert_eq!(exit_code, 99);
        assert_eq!(code, vec!(2,3,0,6,99));
    }

    #[test]
    fn test_big_mul() {
        let mut code = vec!(2,4,4,5,99,0);
        let exit_code = run(&mut code);
        assert_eq!(exit_code, 99);
        assert_eq!(code, vec!(2,4,4,5,99,9801));
    }

    #[test]
    fn test_complex() {
        let mut code = vec!(1,1,1,4,99,5,6,0,99);
        let exit_code = run(&mut code);
        assert_eq!(exit_code, 99);
        assert_eq!(code, vec!(30,1,1,4,2,5,6,0,99));
    }
}