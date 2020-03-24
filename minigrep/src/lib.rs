use std::error::Error;
use std::fs;

#[derive(PartialEq, Debug)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query argument"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing filename"),
        };

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
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
