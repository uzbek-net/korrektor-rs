use pcre::MatchIterator;

fn replace_pairs(input: &str, constant: Box<[(&str, &str)]>) -> String {
    let mut input = input.to_string();

    for (pattern, replacement) in constant.as_ref() {
        let re = regex::Regex::new(pattern).unwrap();
        input = re.replace_all(&input, *replacement).as_ref().to_string();
    }

    input
}

fn wrap_matches(text: &str, matches: MatchIterator) -> String{
    let mut result = text.to_string();

    for m in matches {
        let captured = String::from(m.group(0));
        let replacement = String::from("〈") + &*captured + "〉";
        result = result.replace(&captured, &replacement);
    }

    result
}