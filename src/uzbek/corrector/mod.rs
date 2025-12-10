use crate::utils::wrappers;
use fancy_regex;
use korrektor_rspell::BadWord;
use prelude::*;
use regex;
use serde::{Deserialize, Serialize};

mod prelude;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KorrektorBadWord {
    misspelled: String,
    position: usize,
    suggestions: Vec<String>,
}

pub fn remove_modifiers(text: &str) -> String {
    let mut input = text.to_string();

    let mut re =
        regex::Regex::new(r"\b[A-Z]+[a-z]+[A-Z]+[a-z]*(\b)|(\b)[A-Z]{2,}[a-z]*\b").unwrap();
    input = re.replace_all(&input, "").to_string();

    re = regex::Regex::new(r"-(a|ku|yu|u|da|ya|chi)\b").unwrap();
    input = re.replace_all(&input, "").to_string();

    re = regex::Regex::new(r"-").unwrap();
    input = re.replace_all(&input, " ").to_string();

    re = regex::Regex::new(r"([^a-zA-Z\-‘’])\b").unwrap();
    input = re.replace_all(&input, "${1}").to_string();

    input
}

pub fn correct(text: &str) -> String {
    let mut input: String = text.to_string();

    for (pattern, replacement) in CORRECT.into_iter() {
        let re = regex::Regex::new(pattern).unwrap();
        input = re.replace_all(&input, replacement).to_string();
    }

    let re = regex::Regex::new(r"(\d+)-(январ|феврал|март|апрел|май|июн|июл|август|сентябр|октябр|ноябр|декабр|ЯНВАР|ФЕВРАЛ|МАРТ|АПРЕЛ|МАЙ|ИЮН|ИЮЛ|АВГУСТ|СЕНТЯБР|ОКТЯБР|НОЯБР|ДЕКАБР)").unwrap();
    input = re.replace_all(&input, "${1} ${2}").to_string();
    let re = regex::Regex::new(r"(\d+)-(йил|ЙИЛ|й\.)").unwrap();
    input = re.replace_all(&input, "${1} ${2}").to_string();
    let re = regex::Regex::new(r"\bnuqtai nazar").unwrap();
    input = re.replace_all(&input, "nuqtayi nazar").to_string();
    let re = regex::Regex::new(r"\btarjimai hol").unwrap();
    input = re.replace_all(&input, "tarjimayi hol").to_string();

    input
}

pub fn get_correction_suggestions(text: &str, lang: &str) -> Vec<KorrektorBadWord> {
    // wrap url, mail addresses, and usernames
    let input: String = wrappers::get_wrapped_text(text);

    check_wrapped_text(&input, lang)
}

fn check_wrapped_text(text: &str, lang: &str) -> Vec<KorrektorBadWord> {
    let mut corrections: Vec<KorrektorBadWord> = vec![];

    let re = fancy_regex::Regex::new("([^〈〉](?![^〈]*〉))+").unwrap();
    for capture in re.captures_iter(text) {
        let capture = capture.unwrap()[0].to_string();

        let correction = check_spelling(&capture, lang);

        for rspell_word in correction {
            let korrektor = KorrektorBadWord {
                misspelled: rspell_word.word.to_string(),
                suggestions: rspell_word.suggestions,
                position: rspell_word.offset,
            };

            corrections.push(korrektor);
        }
    }

    corrections
}

fn check_spelling<'a>(text: &'a str, lang: &str) -> Vec<BadWord<'a>> {
    let language = match lang {
        "cyr" => "uz-cyr",
        "lat" => "uz-lat",
        _ => "uz-lat",
    };

    let spell = korrektor_rspell::Spell::new(language).unwrap();

    spell.check(text)
}

#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn remove_modifiers_test() {
        assert_eq!(
            remove_modifiers("stul- stul-ku"),
            String::from("stul  stul")
        );
    }

    #[test]
    fn correct_test() {
        assert_eq!(
            correct("2022-йил 12 yanvar"),
            String::from("2022 йил 12-yanvar")
        );
    }

    #[test]
    fn check_latin_test() {
        let errors_lat: Vec<KorrektorBadWord> = vec![KorrektorBadWord {
            misspelled: "chroyli".to_string(),
            position: 0,
            suggestions: vec![
                "choyli".to_string(),
                "chiroyli".to_string(),
                "chorpoyli".to_string(),
                "choroynali".to_string(),
                "choykorli".to_string(),
                "chiroyi".to_string(),
                "zichroqli".to_string(),
            ],
        }];
        let errors_cyr: Vec<KorrektorBadWord> = vec![KorrektorBadWord {
            misspelled: "чройли".to_string(),
            position: 0,
            suggestions: vec![
                "чойли".to_string(),
                "чиройли".to_string(),
                "чорпойли".to_string(),
                "ойлили".to_string(),
                "ойликчи".to_string(),
                "чоройнали".to_string(),
                "бройлерли".to_string(),
            ],
        }];

        assert_eq!(check_wrapped_text("chroyli", "lat"), errors_lat);
        assert_eq!(check_wrapped_text("chroyli", ""), errors_lat);
        assert_eq!(check_wrapped_text("чройли", "cyr"), errors_cyr);
    }

    #[test]
    fn check_wrapped_text_test() {
        assert_eq!(check_wrapped_text("〈@hello〉 〈〈nyan@mail.uz〉〉 〈〈nya@mail.uz〉〉 〈https://nyan.com〉 go'zal 〈@crystalny〉", "lat"), vec![]);
    }

    #[test]
    fn get_suggestions_test() {
        assert_eq!(
            get_correction_suggestions(
                "@hello nyan@mail.uz nya@mail.uz https://nyan.com go'zal @crystalny",
                "lat"
            ),
            vec![]
        );
    }
}
