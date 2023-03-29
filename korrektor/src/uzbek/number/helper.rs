use korrektor_utils;
use crate::uzbek::number::{float_to_word, integer_to_word};

pub(super) fn is_valid_integer(number: &str) -> bool {
    let re = regex::Regex::new(r"^(\d+)$").unwrap();

    re.is_match(number)
}

pub(super) fn is_valid_float(number: &str) -> bool {
    let re = regex::Regex::new(r"^(\d+\.\d+)$").unwrap();

    re.is_match(number)
}

// wrap ips to preserve from transforming into word equivalent
pub(super) fn wrap_ips(input: &str) -> String {
    // ipv4 addresses regex
    let mut re = pcre::Pcre::compile(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
    let matches_ipv4 = re.matches(input);

    // ipv6 addresses regex
    let mut re =
        pcre::Pcre::compile
            (r"(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|\
            ([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|\
            ([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|\
            ([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|\
            fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|\
            (2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))").unwrap();
    let matches_ipv6 = re.matches(input);

    // wrapping with 〈〉 brackets
    let result = korrektor_utils::wrap_matches(input, matches_ipv4);
    korrektor_utils::wrap_matches(&result, matches_ipv6)
}

pub(super) fn wrap_phones(input : &str) -> String {
    let mut re = pcre::Pcre::compile("(998)(90|91|93|94|95|97|98|99|50|88|69|70|71|72|77|33)([0-9]{3})([0-9]{2})([0-9]{2})").unwrap();
    let matches = re.matches(input);

    korrektor_utils::wrap_matches(input, matches)
}

pub(super) fn convert_floats(text: &str) -> String {
    let mut result = text.to_string();
    let re = fancy_regex::Regex::new(r"(\d+\.\d+)").unwrap();

    for capture in re.captures_iter(text) {
        let capture = capture.unwrap()[0].to_string();

        result = result.replace(&capture, &float_to_word(&capture));
    }

    result
}

pub(super) fn convert_integers(text: &str) -> String {
    let mut result = text.to_string();
    let re = fancy_regex::Regex::new(r"(\d+)").unwrap();

    for capture in re.captures_iter(text) {
        let capture = capture.unwrap()[0].to_string();

        result = result.replace(&capture, &integer_to_word(&capture));
    }

    result
}

#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn wrap_ips_test() {
        let input = "12 124.34.5.234 12.3 2001:db8:3c4d:0015:0000:0000:1a2f:1a2b hello 2001:db8:3c4d:15::";
        let expected = "12 〈124.34.5.234〉 12.3 〈2001:db8:3c4d:0015:0000:0000:1a2f:1a2b〉 hello 〈2001:db8:3c4d:15::〉";

        assert_eq!(wrap_ips(input), expected.to_string());
    }

    #[test]
    fn wrap_phones_test() {
        let input = "salom +998936523602 325 12.3 daraxt 998712345689 71234 336519087";
        let expected = "salom +〈998936523602〉 325 12.3 daraxt 〈998712345689〉 71234 336519087";

        assert_eq!(wrap_phones(input), expected.to_string());
    }

    #[test]
    fn convert_floats_test(){
        assert_eq!(convert_floats("12 12.5 13.1 5"), "12 o‘n ikki butun o‘ndan besh o‘n uch butun o‘ndan bir 5".to_string());
    }
}