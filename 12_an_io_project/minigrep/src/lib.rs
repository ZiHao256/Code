use std::{env, error::Error, fs};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_sensitive() {
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        let query = "Rust";

        assert_eq!(vec!["Rust:"], sensitive_search(query, content));
    }

    #[test]
    fn search_insensitive() {
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me
        ";

        let query = "Rust";

        assert_eq!(
            vec!["Rust:", "        Trust me"],
            insensitive_search(query, content)
        );
    }
}

pub struct Config {
    filename: String,
    query: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new<'a>(mut args: env::Args) -> Result<Config, &'a str> {
        args.next();

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Not found filename"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Not found query"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            filename,
            query,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    println!("the lines containing query is :");

    if config.case_sensitive {
        for line in sensitive_search(&config.query, &content) {
            println!("{}", line);
        }
    } else {
        for line in insensitive_search(&config.query, &content) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn sensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ans: Vec<&str> = Vec::new();

    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            ans.push(line)
        }
    }

    ans
}
