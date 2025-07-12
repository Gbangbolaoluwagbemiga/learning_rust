fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }
    languages.last().unwrap()
}

fn longest_str(languages: &[String]) -> String {
    let mut len = String::new();

    for longest in languages {
        if longest.len() > len.len() {
            len = longest.to_string();
        } else {
            len = len;
        }
    }

    len
}

fn main() {
    let languages = vec![
        String::from("javascript"),
        String::from("rust"),
        String::from("solidity"),
    ];
    let result = next_language(&languages, "javascript");
    let longest = longest_str(&languages);

    println!("The next language is {:?}", result);
    println!("Longest string is {:?}", longest);
}
