pub(super) fn is_valid_integer(number: &str) -> bool {
    let re = regex::Regex::new(r"^(\d+)$").unwrap();

    re.is_match(number)
}

pub(super) fn is_valid_float(number: &str) -> bool {
    let re = regex::Regex::new(r"^(\d+\.\d+)$").unwrap();

    re.is_match(number)
}