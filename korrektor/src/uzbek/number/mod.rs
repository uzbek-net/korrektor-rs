//! Functions to return Uzbek word equivalent of numbers.
//!
//! Only latin mode supported currently.
use crate::uzbek::number::constants::{FLOAT_PREFIX, MULT};
use crate::uzbek::number::helper::{convert_floats, convert_integers, wrap_ips, wrap_phones};
use fancy_regex;

mod constants;
mod helper;

/// Returns a word representation of a given integer number.
///
/// Given an integer string slice returns a String with corresponding word equivalent.
///
/// # Panics
/// - if a string is not a valid integer (make sure you remove any other symbols and whitespace)
/// - if it has more than 18 digits in it (i.e. "999999999999999999" is still valid)
///
/// # Example
/// ```rust
///use korrektor::uzbek::number;
///
/// let output = number::integer_to_word("1024");
/// let expected = "bir ming yigirma to‘rt".to_string();
/// assert_eq!(output, expected);
/// ```
pub fn integer_to_word(number: &str) -> String {
    if !helper::is_valid_integer(number) {
        panic!("Not an integer: {number}");
    }
    if number.len() > 18 {
        panic!("Invalid integer: {number}. Overflow, only numbers under 19 digits are allowed.");
    }

    let number: i64 = number.parse().unwrap();

    //find number to word in constants 0 to 19
    if number == 0 {
        return String::from("nol");
    } else if number < 20 {
        let index = (number - 1) as usize;
        return constants::NUM_1_TO_19[index].1.to_string();
    }
    // find number to word from 0 to 100
    else if number < 100 {
        let index: usize = (number / 10 - 2) as usize;
        return constants::TEEN[index].1.to_string() + " " + &integer_to_word(&(number % 10).to_string());
    }
    // find number to word from 0 to 1000
    else if number < i64::pow(10, 3) {
        return one(number, 2);
    }

    let mut i = 4;
    while i < 27 {
        if number < i64::pow(10, i) { break; }
        i += 1;
    }

    if i % 3 != 0 {
        hundred(number, i - (i % 3))
    } else {
        one(number, i - 3)
    }
}

/// Returns a word representation of a given floating-point number.
///
/// Given an floating-point number string slice returns a String with corresponding word equivalent.
///
/// # Panics
/// - if a string is not a valid floating-point number (exclude integers or use x.0 form, make sure you remove any other symbols and whitespace)
/// - if it has more than 18 digits in its integer or precision part (i.e. "999999999999999999.999999999999999999" is still valid)
///
/// # Example
/// ```rust
///use korrektor::uzbek::number;
///
/// let output = number::float_to_word("574.789");
/// let expected = "besh yuz yetmish to‘rt butun mingdan yetti yuz sakson to‘qqiz".to_string();
/// assert_eq!(output, expected);
/// ```
pub fn float_to_word(number: &str) -> String {
    if !helper::is_valid_float(number) {
        panic!("Invalid floating-point number: {number}");
    }

    let number: Vec<&str> = number.split('.').collect();

    let fraction = number[1];
    if number[1].len() > 18 {
        panic!("Invalid fraction: {fraction}. Overflow, only numbers under 19 digits are allowed.")
    }

    let fraction_prefix = get_fraction_prefix(fraction);

    let integer = integer_to_word(number[0]);
    let fraction = integer_to_word(number[1]);

    let fraction = fraction_prefix + " " + &fraction;

    integer + " butun " + &fraction
}

pub fn all_numbers_to_word(text: &str) -> String {
    let mut input = wrap_ips(text);
    input = wrap_phones(&input);

    // each capture is a part of text outside special brackets (may have multiple words and/or numbers)
    let re = fancy_regex::Regex::new("([^〈〉](?![^〈]*〉))+").unwrap();
    for capture in re.captures_iter(&input.clone()) {
        let initial_cap = capture.unwrap()[0].to_string();
        let mut capture = initial_cap.clone();

        capture = capture.replace(&capture, &convert_floats(&capture));
        input = input.replacen(&initial_cap, &convert_integers(&capture), 1);
    }

    let re = regex::Regex::new("[〈〉]").unwrap();
    input = re.replace_all(&input, "").to_string();

    input
}

fn base(number: i64, power: u32) -> String {
    let base = integer_to_word(&(number / i64::pow(10, power)).to_string());
    let mult_tuple = MULT.iter().find(|x| x.0 == power as i32);
    let mult = match mult_tuple {
        Some(tuple) => tuple.1,
        None => panic!("Such multiplication value is not found! power is {power}")
    };

    base + " " + mult
}

fn one(number: i64, power: u32) -> String {
    let y = number % i64::pow(10, power);
    let s = integer_to_word(&y.to_string());
    let separator =
        if power == 2 && !s.is_empty() { " " } else if y < 100 {
            if y == 0 { "" } else { " " }
        } else { " " };

    base(number, power) + separator + &s
}

fn hundred(number: i64, power: u32) -> String {
    let y = number % i64::pow(10, power);
    let sep = if y < 100 {
        if y == 0 { "" } else { " " }
    } else { " " };

    String::new() + &base(number, power) + sep + &integer_to_word(&y.to_string())
}

fn get_fraction_prefix(number: &str) -> String {
    let prefix = FLOAT_PREFIX[number.len() - 1];

    prefix.to_string()
}


#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn base_test() {
        assert_eq!(base(532, 2), String::from("besh yuz"));
    }

    #[test]
    fn one_test() {
        assert_eq!(one(532, 2), String::from("besh yuz o‘ttiz ikki"));
    }

    #[test]
    fn hundred_test() {
        assert_eq!(hundred(3456, 3), String::from("uch ming to‘rt yuz ellik olti"));
    }

    #[test]
    fn cw_test() {
        assert_eq!(integer_to_word("0"), String::from("nol"));
        assert_eq!(integer_to_word("9"), String::from("to‘qqiz"));
        assert_eq!(integer_to_word("32"), String::from("o‘ttiz ikki"));
        assert_eq!(integer_to_word("104"), String::from("bir yuz to‘rt"));
        assert_eq!(integer_to_word("1024"), String::from("bir ming yigirma to‘rt"));
        assert_eq!(integer_to_word("3456"), String::from("uch ming to‘rt yuz ellik olti"));
    }

    #[test]
    fn float_test() {
        assert_eq!(float_to_word("3.0"), String::from("uch butun o‘ndan nol"));
        assert_eq!(float_to_word("3.75"), String::from("uch butun yuzdan yetmish besh"));
        assert_eq!(float_to_word("3.754"), String::from("uch butun mingdan yetti yuz ellik to‘rt"));
        assert_eq!(float_to_word("3.7548"), String::from("uch butun o‘n mingdan yetti ming besh yuz qirq sakkiz"))
    }

    #[test]
    fn all_numbers_test() {
        let input = "12 998336523409 12.5 1024 124.34.5.234 2001:db8:3c4d:0015:0000:0000:1a2f:1a2b";
        let expected = "o‘n ikki 998336523409 o‘n ikki butun o‘ndan besh bir ming yigirma to‘rt 124.34.5.234 2001:db8:3c4d:0015:0000:0000:1a2f:1a2b";

        assert_eq!(all_numbers_to_word(input), expected.to_string());
    }
}
