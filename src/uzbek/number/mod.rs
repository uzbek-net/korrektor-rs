//! Functions to return Uzbek word equivalent of numbers.
//!
//! Only latin mode supported currently.
use fancy_regex;
use crate::error::KorrektorError;

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
/// let output = number::integer_to_word("1024").unwrap();
/// let expected = "bir ming yigirma to‘rt".to_string();
/// assert_eq!(output, expected);
/// ```
pub fn integer_to_word(number: &str) -> Result<String, KorrektorError> {
    if !helper::is_valid_integer(number) {
        return Err(KorrektorError::InvalidNumber(
            number.to_string(), "Not a valid integer".to_string())
        )
    }
    if number.len() > 18 {
        return Err(KorrektorError::InvalidNumber(
            number.to_string(), "Integer should not contain more than 18 digits".to_string())
        )
    }

    let number: i64 = number.parse().unwrap();

    match number {
        0 => Ok(String::from("nol")),
        1..=19 => {
            let index = (number - 1) as usize;
            Ok(constants::NUM_1_TO_19[index].1.to_string())
        },
        20..=99 => {
            let index: usize = (number / 10 - 2) as usize;
            Ok(constants::TEEN[index].1.to_string() + " " + &integer_to_word(&(number % 10).to_string())?)
        },
        100..=999 => one(number, 2),
        _ => {
            let mut i = 4;
            while i < 27 {
                if number < i64::pow(10, i) { break; }
                i += 1;
            }

            if i % 3 != 0 {
                Ok(hundred(number, i - (i % 3))?)
            } else {
                Ok(one(number, i - 3)?)
            }
        }
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
/// let output = number::float_to_word("574.789").unwrap();
/// let expected = "besh yuz yetmish to‘rt butun mingdan yetti yuz sakson to‘qqiz".to_string();
/// assert_eq!(output, expected);
/// ```
pub fn float_to_word(number: &str) -> Result<String, KorrektorError> {
    if !helper::is_valid_float(number) {
        return Err(KorrektorError::InvalidNumber(
            number.to_string(), "Not a valid floating-point number".to_string())
        )
    }

    let number: Vec<&str> = number.split('.').collect();

    let fraction = number[1];
    if number[1].len() > 18 {
        return Err(KorrektorError::InvalidNumber(
            number[1].to_string(), "Precision part should not contain more than 18 digits".to_string())
        )
    }

    let fraction_prefix = get_fraction_prefix(fraction);

    let integer = integer_to_word(number[0])?;
    let fraction = integer_to_word(number[1])?;

    let fraction = fraction_prefix + " " + &fraction;

    Ok(integer + " butun " + &fraction)
}

/// Converts all numbers in text into their word representation.
///
/// Given a string slice returns a String with all numbers
/// converted into their word equivalent
/// (excluding IP addresses and Uzbekistan phone numbers in their full form)
///
/// # Example
/// ```rust
/// use korrektor::uzbek::number;
///
/// let output = number::numbers_to_word("12, salom 998336523409 12.5 daraxt 1024 124.34.5.234").unwrap();
/// let expected = "o‘n ikki, salom 998336523409 o‘n ikki butun o‘ndan besh daraxt bir ming yigirma to‘rt 124.34.5.234".to_string();
/// assert_eq!(output, expected);
/// ```
pub fn numbers_to_word(text: &str) -> Result<String, KorrektorError> {
    let mut input = helper::wrap_ips(text);
    input = helper::wrap_phones(&input);

    // each capture is a part of text outside special brackets (may have multiple words and/or numbers)
    let re = fancy_regex::Regex::new("([^〈〉](?![^〈]*〉))+").unwrap();
    for capture in re.captures_iter(&input.clone()) {
        let initial_cap = capture.unwrap()[0].to_string();
        let mut capture = initial_cap.clone();

        capture = capture.replace(&capture, &helper::convert_floats(&capture)?);
        input = input.replacen(&initial_cap, &helper::convert_integers(&capture)?, 1);
    }

    let re = regex::Regex::new("[〈〉]").unwrap();
    input = re.replace_all(&input, "").to_string();

    Ok(input)
}

fn base(number: i64, power: u32) -> Result<String, KorrektorError> {
    let base = integer_to_word(&(number / i64::pow(10, power)).to_string())?;
    let mult_tuple = constants::MULT.iter().find(|x| x.0 == power as i32);
    let mult = match mult_tuple {
        Some(tuple) => tuple.1,
        None => panic!("Such multiplication value is not found! power is {power}")
    };

    Ok(base + " " + mult)
}

fn one(number: i64, power: u32) -> Result<String, KorrektorError> {
    let y = number % i64::pow(10, power);
    let s = integer_to_word(&y.to_string())?;
    let separator =
        if power == 2 && !s.is_empty() { " " } else if y < 100 {
            if y == 0 { "" } else { " " }
        } else { " " };

    Ok(base(number, power)? + separator + &s)
}

fn hundred(number: i64, power: u32) -> Result<String, KorrektorError> {
    let y = number % i64::pow(10, power);
    let sep = if y < 100 {
        if y == 0 { "" } else { " " }
    } else { " " };

    Ok(String::new() + &base(number, power)? + sep + &integer_to_word(&y.to_string())?)
}

fn get_fraction_prefix(number: &str) -> String {
    let prefix = constants::FLOAT_PREFIX[number.len() - 1];

    prefix.to_string()
}


#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn base_test() {
        assert_eq!(base(532, 2).unwrap(), String::from("besh yuz"));
    }

    #[test]
    fn one_test() {
        assert_eq!(one(532, 2).unwrap(), String::from("besh yuz o‘ttiz ikki"));
    }

    #[test]
    fn hundred_test() {
        assert_eq!(hundred(3456, 3).unwrap(), String::from("uch ming to‘rt yuz ellik olti"));
    }

    #[test]
    fn cw_test() {
        assert_eq!(integer_to_word("0").unwrap(), String::from("nol"));
        assert_eq!(integer_to_word("9").unwrap(), String::from("to‘qqiz"));
        assert_eq!(integer_to_word("32").unwrap(), String::from("o‘ttiz ikki"));
        assert_eq!(integer_to_word("104").unwrap(), String::from("bir yuz to‘rt"));
        assert_eq!(integer_to_word("1024").unwrap(), String::from("bir ming yigirma to‘rt"));
        assert_eq!(integer_to_word("3456").unwrap(), String::from("uch ming to‘rt yuz ellik olti"));
    }

    #[test]
    fn float_test() {
        assert_eq!(float_to_word("3.0").unwrap(), String::from("uch butun o‘ndan nol"));
        assert_eq!(float_to_word("3.75").unwrap(), String::from("uch butun yuzdan yetmish besh"));
        assert_eq!(float_to_word("3.754").unwrap(), String::from("uch butun mingdan yetti yuz ellik to‘rt"));
        assert_eq!(float_to_word("3.7548").unwrap(), String::from("uch butun o‘n mingdan yetti ming besh yuz qirq sakkiz"))
    }

    #[test]
    fn all_numbers_test() {
        let input = "12, 998336523409 12.5 1024 124.34.5.234 2001:db8:3c4d:0015:0000:0000:1a2f:1a2b 12.5 1024";
        let expected = "o‘n ikki, 998336523409 o‘n ikki butun o‘ndan besh bir ming yigirma to‘rt 124.34.5.234 2001:db8:3c4d:0015:0000:0000:1a2f:1a2b o‘n ikki butun o‘ndan besh bir ming yigirma to‘rt";

        assert_eq!(numbers_to_word(input).unwrap(), expected.to_string());
    }
}
