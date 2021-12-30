use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
        where T: Iterator<Item=String> 
    {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };
        let case_sensitive = Config::is_case_sensitive(&mut args);

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    // Checks ENV and cmd args
    // Cmd args take precedence
    fn is_case_sensitive<T>(args: &mut T) -> bool 
        where T: Iterator<Item=String>
    {
        if let Some(v) = args.next() {
            if v == "-c" {
                return false;
            }
        };

        env::var("CASE_INSENSITIVE").is_err()
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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
        let config = Config::new(args.into_iter()).unwrap();

        assert_eq!("query", config.query);
        assert_eq!("filename", config.filename)
    }

    #[test]
    fn confignew_passing_too_little_arguments_returns_error() {
        let args = [String::from("exe"), String::from("query")];
        let result = Config::new(args.into_iter());

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
        let config = Config::new(args.into_iter()).unwrap();

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
        let config = Config::new(args.into_iter()).unwrap();

        if let Err(_) = run(config) {
            panic!("An existing file was passed so result should be Ok")
        }
    }

    #[test]
    fn search_return_one_result_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_case_insensitive_return_results() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
