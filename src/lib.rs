mod day1;

use std::fs;

pub struct Config {
    day: String,
    file: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("too few arguments");
        }

        Ok(Config {
            day: args[1].to_string(),
            file: args[2].to_string(),
        })
    }
}

pub fn run(config: &Config) -> Result<(), &'static str> {
    let contents = match fs::read_to_string(&config.file) {
        Ok(contents) => contents,
        Err(_) => return Err("Could not read file"),
    };

    let results: DayResults = match config.day.as_str() {
        "day1" => day1::run(&contents)?,
        _ => return Err("Invalid day"),
    };

    println!("Part 1: {}", results.part1);
    if results.part2.is_some() {
        println!("Part 2: {}", results.part2.unwrap());
    }

    Ok(())
}

pub struct DayResults {
    part1: u128,
    part2: Option<u128>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_config_correctly() {
        let args = vec![
            String::from("target/debug/bin"),
            String::from("day1"),
            String::from("day1/input.txt"),
        ];

        let config = Config::build(&args).unwrap();

        assert_eq!(config.day, "day1");
        assert_eq!(config.file, "day1/input.txt")
    }

    #[test]
    fn should_return_err_if_not_enough_arguments() {
        let args = vec![
            String::from("target/debug/bin"),
            String::from("day"),
        ];

        let config = Config::build(&args);

        assert!(config.is_err());
        assert_eq!(config.err().unwrap(), "too few arguments");
    }
}
