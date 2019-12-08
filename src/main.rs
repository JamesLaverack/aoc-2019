extern crate clap;

mod fuel;
mod intcode;

use std::path::Path;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Advent of Code 2019 Solver")
        .version("0.1.0")
        .author("James Laverack <james@jameslaverack.com>")
        .about("Code solutions to Advent of Code 2019")
        .subcommand(SubCommand::with_name("day1a")
            .about("Day one, part A")
            .arg(Arg::with_name("MODULES_FILE")
                .required(true)
                .index(1)
                .help("Path to file with module masses")))
        .subcommand(SubCommand::with_name("day1b")
            .about("Day one, part B")
            .arg(Arg::with_name("MODULES_FILE")
                .required(true)
                .index(1)
                .help("Path to file with module masses")))
        .subcommand(SubCommand::with_name("day2a")
            .about("Day two, part A")
            .arg(Arg::with_name("CODE_FILE")
                .required(true)
                .index(1)
                .help("Path to file with intcode instructions")))
        .subcommand(SubCommand::with_name("day2b")
            .about("Day two, part B")
            .arg(Arg::with_name("CODE_FILE")
                .required(true)
                .index(1)
                .help("Path to file with intcode instructions")))
        .get_matches();

    println!("Advent of Code 2019");

    match matches.subcommand_name() {
        Some("day1a") => {
            let file_name = matches
                .subcommand_matches("day1a")
                .unwrap()
                .value_of("MODULES_FILE")
                .unwrap();
            let file_path = Path::new(file_name);

            println!("Executing day one, part A on file {}", file_path.display());

            let modules = match fuel::load_modules(file_path) {
                Err(why) => panic!("Failed to load modules: {}", why),
                Ok(total) => total,
            };
            let total_fuel :u32 = modules.into_iter().map(fuel::fuel_requirement).sum();
            println!("Total fuel requirement is {}", total_fuel)
        }
        Some("day1b") => {
            let file_name = matches
                .subcommand_matches("day1b")
                .unwrap()
                .value_of("MODULES_FILE")
                .unwrap();
            let file_path = Path::new(file_name);

            println!("Executing day one, part B on file {}", file_path.display());

            let modules = match fuel::load_modules(file_path) {
                Err(why) => panic!("Failed to load modules: {}", why),
                Ok(total) => total,
            };
            let total_fuel :u32 = modules.into_iter().map(fuel::recursive_fuel_requirement).sum();
            println!("Total fuel requirement is {}", total_fuel)
        }
        Some("day2a") => {
            let file_name = matches
                .subcommand_matches("day2a")
                .unwrap()
                .value_of("CODE_FILE")
                .unwrap();
            let file_path = Path::new(file_name);

            println!("Executing day two, part A on file {}", file_path.display());

            let mut code = match intcode::load(file_path) {
                Err(why) => panic!("Failed to load code: {}", why),
                Ok(total) => total,
            };
            intcode::run(&mut code);
            println!("Value at position zero is {}", code[0])
        }
        Some("day2b") => {
            let file_name = matches
                .subcommand_matches("day2b")
                .unwrap()
                .value_of("CODE_FILE")
                .unwrap();
            let file_path = Path::new(file_name);

            println!("Executing day two, part B on file {}", file_path.display());

            let code = match intcode::load(file_path) {
                Err(why) => panic!("Failed to load code: {}", why),
                Ok(total) => total,
            };
            for noun in 0..100 {
                for verb in 0..100 {
                    let mut instance = code.to_vec();
                    instance[1] = noun;
                    instance[2] = verb;
                    let exit_code = intcode::run(&mut instance);
                    if exit_code != 99 {
                        panic!("Found error code {}", exit_code);
                    }

                    const EXPECTED:u32 = 19690720;
                    if instance[0] == EXPECTED {
                        println!("Found solution. Noun {}, verb {}. Puzzle answer {}",
                                 noun,
                                 verb,
                                 100 * noun + verb);
                        return;
                    }
                }
            }
        }
        None => println!("Please use a command"),
        _ => unreachable!()
    }
}
