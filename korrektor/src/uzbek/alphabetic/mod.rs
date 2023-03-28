//! Functions to sort Uzbek words.
//!
//! Both cyrillic and latin modes can be used.
use korrektor_utils;

mod constants;

/// Sorts words in alphabetically ascending order.
///
/// Given String of text returns a new String with words sorted and separated with a newline.
///
/// # Example
/// ```rust
/// use korrektor::uzbek::alphabetic;
///
/// let output = alphabetic::sort("G‘ozal estafeta chilonzor o'zbek chiroyli");
/// let expected = "estafeta o‘zbek chilonzor chiroyli G‘ozal".to_string();
/// assert_eq!(output, expected);
///```
pub fn sort(text: &str) -> String {
    // replace complex symbols in text with sortable alternatives
    let sortable = &to_sortable(text.to_string());

    let sorted_intermediate = sort_sortable(sortable);

    // replace sortable alternatives with original values after sorting
    from_sortable(sorted_intermediate)
}

fn to_sortable(text: String) -> String {
    let mut input: String = text;

    input = korrektor_utils::replace_pairs(&input, Box::new(constants::TO_SORT));

    input
}

fn from_sortable(text: String) -> String {
    let mut input: String = text;

    input = korrektor_utils::replace_pairs(&input, Box::new(constants::FROM_SORT));

    input
}

fn usort(string1: &str, string2: &str) -> i8 {
    let length = std::cmp::min(string1.len() - 1, string2.len() - 1);

    for i in 0..length {
        let char1 = match string1.chars().nth(i) {
            Some(char) => char,
            None => panic!("Error in usort: no char at the index {i} in &str: {string1}")
        };
        let char2 = match string2.chars().nth(i) {
            Some(char) => char,
            None => panic!("Error in usort: no char at the index {i} in &str: {string2}")
        };

        // get position of characters in the alphabet
        let value1 = get_value(char1);
        let value2 = get_value(char2);

        match value1.cmp(&value2) {
            std::cmp::Ordering::Less => return -1,
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Equal => continue,
        };
    }

    match (string1.len()).cmp(&string2.len()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Equal => 0,
    }
}

fn sort_sortable(text: &str) -> String {
    let mut sortable: Vec<&str> = text.split_whitespace().collect();
    let mut len = sortable.len();

    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..len - 1 {
            match usort(sortable[i], sortable[i + 1]) {
                1 => {
                    sortable.swap(i, i + 1);
                    sorted = false;
                }
                _ => continue,
            }
        }
        len -= 1;
    }

    let mut result = String::new();
    for word in sortable {
        result.push_str(word);
        result.push(' ');
    }

    result.trim().to_string()
}

fn is_exceptioned(value: char) -> bool {
    if value == 'Ö' || value == 'Ü' { return true; }

    false
}

fn get_exceptioned_value(value: char) -> usize {
    if value == 'Ö' { return 55; }
    if value == 'Ü' { return 56; }

    0
}

fn get_value(value: char) -> usize {
    if is_exceptioned(value) {
        get_exceptioned_value(value)
    } else {
        return match constants::CHAR_ORDER.iter().position(|&r| r == value.to_string()) {
            Some(num) => num,
            None => panic!("Error in usort: char {value} is not found and can not be sorted")
        };
    }
}

#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn to_sortable_test() {
        let result = to_sortable("G'g' O'o' ShSHsh ChCHch ʻʼ'‘’‛′ʽ`".to_string());
        assert_eq!(result, "Ğğ Ŏŏ ŠÖš ČÜč ʼʼʼʼʼʼʼʼʼ");
    }

    #[test]
    fn from_sortable_test() {
        let result = from_sortable("Ğğ Ŏŏ ŠÖš ČÜč".to_string());
        assert_eq!(result, "G‘g‘ O‘o‘ ShSHsh ChCHch");
    }

    #[test]
    fn is_exceptioned_test() {
        assert!(is_exceptioned('Ö'));
        assert!(is_exceptioned('Ü'));
    }

    #[test]
    fn get_exceptioned_value_test() {
        assert_eq!(get_exceptioned_value('Ö'), 55);
        assert_eq!(get_exceptioned_value('Ü'), 56);
    }

    #[test]
    fn usort_test() {
        assert_eq!(usort("čiroyli", "čilonzor"), 1);
        assert_eq!(usort("čiroyli", "čiroyli"), 0);
        assert_eq!(usort("čilonzor", "čiroyli"), -1);
    }

    #[test]
    fn get_sorted_text_test() {
        let input = "G‘ozal estafeta chilonzor o'zbek chiroyli";
        let output = String::from("estafeta o‘zbek chilonzor chiroyli G‘ozal");
        assert_eq!(sort(input), output)
    }

    #[test]
    fn get_sorted_text_cyr_test() {
        let input = "Ғозал эстафета чилонзор ўзбек чиройли";
        let output = String::from("чилонзор чиройли эстафета ўзбек Ғозал");
        assert_eq!(sort(input), output)
    }
}