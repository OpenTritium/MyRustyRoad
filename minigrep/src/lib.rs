use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    search_path: String,
}

impl FromIterator<String> for Config {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut iter = iter.into_iter().skip(1);
        let first = iter.next();
        let second = iter.next();
        match (first, second) {
            (Some(query), Some(search_path)) => Config { query, search_path },
            _ => panic!("failed to convert"),
        }
    }
}

pub fn run(config: Config, is_ignore_case: bool) -> Result<(), Box<dyn std::error::Error>> {
    let file_context = fs::read_to_string(&config.search_path)?;
    let result = if is_ignore_case {
        search_case_insensitive(&config.query, &file_context)
    } else {
        search(&config.query, &file_context)
    };
    if result.is_empty() {
        println!("not found");
    } else {
        println!("{:?}", result);
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|&line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn search_sensetive() {
        let query = "best";
        let file_content = indoc! {"
        helix is 
        the best
        text editor 
        ever"};
        assert_eq!(vec!["the best"], search(query, file_content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = indoc! {"
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me."
        };
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
