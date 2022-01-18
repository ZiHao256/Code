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
    pub fn new(args: &Vec<String>) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Not Enough Arguments"));
        }

        let filename = args[1].clone();
        let query = args[2].clone();

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

    if config.case_sensitive{
        for line in sensitive_search(&config.query, &content) {
            println!("{}", line);
        }        
    }else{
        for line in insensitive_search(&config.query, &content) {
            println!("{}", line);
        }      
    }


    Ok(())
}

fn sensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ans: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            ans.push(line);
        }
    }

    ans
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
