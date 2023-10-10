use colored::Colorize;
use std::{fs, process};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("参数缺失");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = match args.get(3) {
            Some(v) => v.as_str().cmp("-c") == std::cmp::Ordering::Equal,
            None => false,
        };
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) {
    let content = match fs::read_to_string(config.filename) {
        Ok(content) => content,
        Err(error) => {
            println!(
                "{} {}",
                "Error:".red().bold(),
                error.to_string().as_str().red().bold()
            );
            process::exit(0);
        }
    };
    let lines;
    if config.case_sensitive {
        lines = search_case_insensitive(&config.query, &content);
    } else {
        lines = search(&config.query, &content);
    }
    if lines.len() == 0 {
        println!("{}", "未找到内容".red().bold());
        process::exit(0);
    }
    for line in lines {
        println!("{}", line.green().bold());
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "lucius";
        let contents = "
wangshi
peter
lucius
boss
      ";
        assert_eq!(vec!["lucius"], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "lucius";
        let contents = "
wangshi
Lucius
peter
luciUs
        ";
        assert_eq!(
            vec!["Lucius", "luciUs"],
            search_case_insensitive(query, contents)
        );
    }
}
