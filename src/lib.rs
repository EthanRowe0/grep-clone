use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "grepclone")]
#[command(about = "A simple grep clone")]
pub struct Config {
    pub query: String,
    pub filename: String,

    #[arg(long)]
    pub ignore_case: bool,
    
    #[arg(long)]
    pub whole_word:bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents, config.whole_word)
    } else {
        search(&config.query, &contents, config.whole_word)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, whole_word: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            if whole_word {
                line.split_whitespace().any(|word| word == query)
            } else {
                line.contains(query)
            }
        })
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str, whole_word: bool) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| {
            if whole_word {
                line.split_whitespace().any(|word| word.to_lowercase() == query)
            } else {
                line.to_lowercase().contains(&query)
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "exist";
        let contents = "/
Some multiline
text that
should exist.";

        assert_eq!(
            vec!["should exist."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "IN";
        let contents = "/
SOME strange
CASES iN thiS
muLTILinE tExt
that sHouLD
exist.";

        assert_eq!(
            vec!["CASES iN thiS", "muLTILinE tExt"],
            search_case_insensitive(query, contents)
        );
    }
}
