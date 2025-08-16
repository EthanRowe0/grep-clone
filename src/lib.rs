use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(name = "grepclone")]
#[command(about = "A simple grep clone")]
#[command(version)]
pub struct Config {
    pub query: String,
    pub filename: String,

    #[arg(long)]
    pub ignore_case: bool,
    
    #[arg(long)]
    pub whole_word:bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(&config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = search(&config, &contents);

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(config: &Config, contents: &'a str) -> Vec<&'a str> {
    let pattern = if config.whole_word {
        format!(r"\b{}\b", regex::escape(&config.query))
    } else {
        regex::escape(&config.query)
    };

    let re = if config.ignore_case {
        Regex::new(&format!("(?i){}", pattern)).unwrap()
    } else {
        Regex::new(&pattern).unwrap()
    };

    contents
        .lines()
        .filter(|line| re.is_match(line))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive_partial() {
        let config = Config {
            query: "he".to_string(),
            filename: "".to_string(),
            ignore_case: false,
            whole_word: false,
        };

        let contents = "He clasps the crag
Close to the sun
he stands";

        assert_eq!(
            vec!["He clasps the crag", "Close to the sun", "he stands"],
            search(&config, contents)
        );
    }

    #[test]
    fn case_sensitive_whole_word() {
        let config = Config {
            query: "he".to_string(),
            filename: "".to_string(),
            ignore_case: false,
            whole_word: true,
        };

        let contents = "He clasps the crag
Close to the sun
he stands";

        assert_eq!(
            vec!["he stands"],
            search(&config, contents)
        );
    }

    #[test]
    fn case_insensitive_partial() {
        let config = Config {
            query: "he".to_string(),
            filename: "".to_string(),
            ignore_case: true,
            whole_word: false,
        };

        let contents = "He clasps the crag
Close to the sun
he stands";

        assert_eq!(
            vec!["He clasps the crag", "Close to the sun", "he stands"],
            search(&config, contents)
        );
    }

    #[test]
    fn case_insensitive_whole_word() {
        let config = Config {
            query: "he".to_string(),
            filename: "".to_string(),
            ignore_case: true,
            whole_word: true,
        };

        let contents = "He clasps the crag
Close to the sun
he stands";

        assert_eq!(
            vec!["He clasps the crag", "he stands"],
            search(&config, contents)
        );
    }

    #[test]
    fn whole_word_with_punctuation() {
        let config = Config {
            query: "sun".to_string(),
            filename: "".to_string(),
            ignore_case: false,
            whole_word: true,
        };

        let contents = "Close to the sun.
he stands, admiring the sunset,
it is beautiful.";

        assert_eq!(
            vec!["Close to the sun."],
            search(&config, contents)
        );
    }
}
