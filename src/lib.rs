use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
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
    fn confignew_passing_proper_args_crates_config() {
        let args = [
            String::from("exe"),
            String::from("query"),
            String::from("filename"),
        ];
        let config = Config::new(&args).unwrap();

        assert_eq!("query", config.query);
        assert_eq!("filename", config.filename)
    }

    #[test]
    fn confignew_passing_too_little_arguments_returns_error() {
        let args = [String::from("exe"), String::from("query")];
        let result = Config::new(&args);

        if let Ok(_) = result {
            panic!("The returned result should be invalid");
        }
    }

    #[test]
    fn run_passing_nonexisting_file_returns_error() {
        let args = [
            String::from("exe"),
            String::from("query"),
            String::from("nonexistingfile.txt"),
        ];
        let config = Config::new(&args).unwrap();

        if let Ok(()) = run(config) {
            panic!("A non-existing file was passed so result should not be Ok")
        }
    }

    #[test]
    fn run_passing_existing_file_returns_ok() {
        let args = [
            String::from("exe"),
            String::from("query"),
            String::from("poem.txt"),
        ];
        let config = Config::new(&args).unwrap();

        if let Err(_) = run(config) {
            panic!("An existing file was passed so result should be Ok")
        }
    }

    #[test]
    fn search_return_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
