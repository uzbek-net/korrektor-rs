/// Crate that holds utility functions needed
/// for korrektor.
use pcre::MatchIterator;

/// Replaces constants represented as a tuple array. Each first
/// element of the tuple replaces each second element
/// with the help of the regex crate.
pub fn replace_pairs(input: &str, constant: Box<[(&str, &str)]>) -> String {
    let mut input = input.to_string();

    for (pattern, replacement) in constant.as_ref() {
        let re = regex::Regex::new(pattern).unwrap();
        input = re.replace_all(&input, *replacement).as_ref().to_string();
    }

    input
}

/// Wraps all regex matches of pcre crate
/// in 〈〉 brackets in order to preserve some
/// text from some operations in korrektor.
pub fn wrap_matches(text: &str, matches: MatchIterator) -> String{
    let mut result = text.to_string();

    for m in matches {
        let captured = String::from(m.group(0));
        let replacement = String::from("〈") + &*captured + "〉";
        result = result.replace(&captured, &replacement);
    }

    result
}