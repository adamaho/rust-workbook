use std::error::Error;
use std::fs;

#[derive(PartialEq, Debug)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments passed");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // question mark operator is used here to return the error directly as the result.
    // this to me is more confusing then using unwrap or at match
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_config() {
        let args: Vec<String> = vec![
            String::from("executer"),
            String::from("foo"),
            String::from("bar.txt"),
        ];

        let config = Config::new(&args).unwrap();

        assert_eq!(Config { query: String::from("foo"), filename: String::from("bar.txt") }, config);
    }

    #[test]
    fn it_returns_error_when_too_few_args() {
        let args: Vec<String> = vec![String::from("foo"), String::from("bar.txt")];

        let config = Config::new(&args);

        assert!(config.is_err());
    }

    #[test]
    fn it_returns_one_result() {
        let query = "three";
        let contents = "\
Rust:
safe, fast, secure.
Pick three
";

        assert_eq!(vec!["Pick three"], search(query, contents));
    }
}
