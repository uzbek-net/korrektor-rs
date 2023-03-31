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

/// Wrap all matches in the given text with
/// provided pattern that must be a valid regex.
///
/// # Panics
/// Supplied an invalid regex that can not be compiled by pcre crate
pub fn wrap_regex(text: &str, pattern: &str) -> String {
    let mut re = pcre::Pcre::compile(pattern).unwrap();
    let matches = re.matches(text);

    wrap_matches(text, matches)
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

/// Removes all brackets that wrap special text
/// protected from korrektor operations.
pub fn unwrap_text(text: &str) -> String {
    let re = regex::Regex::new("[〈〉]").unwrap();

    re.replace_all(text, "").to_string()
}

#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn unwrap_text_test() {
        let input = "@ki-d @ki- 〈@hello〉 〈〈nyan@mail.uz〉〉 〈〈nya@mail.uz〉〉 〈https://nyan.com〉 go'zal 〈@crystalny〉";
        let expected = "@ki-d @ki- @hello nyan@mail.uz nya@mail.uz https://nyan.com go'zal @crystalny";

        assert_eq!(unwrap_text(input), expected.to_string());
    }

    #[test]
    fn wrap_regex_test() {
        let input = "@ki-d @ki- @hello nyan@mail.uz nya@mail.uz https://nyan.com go'zal @crystalny";
        let pattern = r"([\w-]+(?:\.[\w-]+)*)@((?:[\w-]+\.)*\w[\w-]{0,66})\.([a-z]{2,6}(?:\.[a-z]{2})?)|@(?!.*\-|.*\-$)[a-zA-Z0-9][\w-]+[a-zA-Z0-9]{0,39}";
        let expected = "@ki-d @ki- 〈@hello〉 〈nyan@mail.uz〉 〈nya@mail.uz〉 https://nyan.com go'zal 〈@crystalny〉";

        assert_eq!(wrap_regex(input, pattern), expected.to_string());

        let input = "@ki-d https://nyan.com go'zal @crystalny";
        let pattern = "(?i)\\b(?:(?:https?|ftp|file|ssh):\\/\\/|www\\.|ftp\\.)[-A-Z0-9+&@#\\/%=~_|$?!:,.]*[A-Z0-9+&@#\\/%=~_|$]";
        let expected = "@ki-d 〈https://nyan.com〉 go'zal @crystalny";

        assert_eq!(wrap_regex(input, pattern), expected.to_string());
    }
}