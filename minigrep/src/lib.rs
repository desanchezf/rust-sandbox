/// # Examples
/// ```
/// use minigrep::search;
///
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// let result = search("duct", contents);
/// assert_eq!(result, vec!["safe, fast, productive."]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(
        |line| line.to_lowercase().contains(
            &query.to_lowercase()
        )
    )
    .collect()
    // El collect() es una lista que va almacenando los valores que se van filtrando.
}

fn highlight_line(line: &str, query: &str, ignore_case: bool) -> String {
    if query.is_empty() {
        return line.to_string();
    }

    if !ignore_case {
        return line.replace(query, &format!("\x1b[31m{query}\x1b[0m"));
    }

    let query_lower = query.to_lowercase();
    let chars: Vec<char> = line.chars().collect();
    let query_len = query.chars().count();
    let mut highlighted = String::new();
    let mut index = 0;

    while index < chars.len() {
        if index + query_len <= chars.len() {
            let slice: String = chars[index..index + query_len].iter().collect();
            if slice.to_lowercase() == query_lower {
                highlighted.push_str(&format!("\x1b[31m{slice}\x1b[0m"));
                index += query_len;
                continue;
            }
        }

        highlighted.push(chars[index]);
        index += 1;
    }

    highlighted
}

pub fn print_matches(query: &str, matches: &[&str], ignore_case: bool) {
    if matches.is_empty() {
        println!("No matches found");
        return;
    }

    for (i, line) in matches.iter().enumerate() {
        let highlighted = highlight_line(line, query, ignore_case);
        println!("Coincidence ({}): {}", i + 1, highlighted);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
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
