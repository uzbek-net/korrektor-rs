pub(crate) mod prelude;
use crate::utils::pcre as ipcre;
use crate::utils::wrappers;
use inflector::Inflector;

pub fn to_cyrillic(text: String) -> String {
    let mut input = text;

    input = ipcre::replace_pairs(&input, prelude::TO_CYR);

    let re = regex::Regex::new("([a-zA-Zа-яА-ЯўқғҳЎҚҒҲʼʻ?!.0-9])(“|”|\")").unwrap();
    input = re.replace_all(&input, "$1»").as_ref().to_string();

    input = ipcre::replace_pairs(&input, prelude::PRE_RETRANSLIT);

    // replace constant words from constants::RETRANSLIT
    input = retransliterate(&input);

    input = ipcre::replace_pairs(&input, prelude::POST_RETRANSLIT);
    input = ipcre::replace_pairs(&input, prelude::TO_CYR_SUB);
    input = replace_letters(&input, prelude::LATIN_L2C, prelude::CYRILLIC_L2C);
    input = ipcre::replace_pairs(&input, prelude::TO_CYR_CORRECT);

    input
}

pub fn to_latin(text: String) -> String {
    let mut input = text;

    input = ipcre::replace_pairs(&input, prelude::PRE_TO_LATIN);

    input = replace_letters(&input, prelude::CYRILLIC_C2L, prelude::LATIN_C2L);

    input = ipcre::replace_pairs(&input, prelude::TO_LATIN);

    input
}

pub fn to(text: String, alphabet: &str) -> String {
    // wrapping special text with 〈〉 to avoid correction and transliteration (emails, usernames, URLs)
    let mut input = wrappers::get_wrapped_text(&text);

    // transliterate to specified language passed as argument
    match alphabet {
        "cyr" => input = to_cyrillic(input),
        "lat" => input = to_latin(input),
        _ => input = to_latin(input),
    }

    // unwrap special text
    let re = regex::Regex::new("[〈〉]").unwrap();
    input = re.replace_all(&input, "").to_string();

    input
}

fn replace_letters(input: &str, input_alphabet: &[&str], output_alphabet: &[&str]) -> String {
    let mut input = input.to_string();

    let mut i = 0;
    while i < input_alphabet.len() {
        let re = regex::Regex::new(input_alphabet[i]).unwrap();
        input = re
            .replace_all(&input, output_alphabet[i])
            .as_ref()
            .to_string();
        i += 1;
    }

    input
}

fn retransliterate(input: &str) -> String {
    let mut input = input.to_string();

    for (pattern, replacement) in prelude::RETRANSLIT {
        let wrapped = String::from("\\b") + pattern;

        let text = input.clone();
        let re = regex::Regex::new(&wrapped).unwrap();
        for cap in re.captures_iter(&text) {
            if cap[0] == cap[0].to_string().to_lowercase() {
                input = re.replace(&input, *replacement).parse().unwrap();
            } else if cap[0] == cap[0].to_string().to_uppercase() {
                input = re
                    .replace(&input, replacement.to_uppercase())
                    .parse()
                    .unwrap();
            } else if cap[0] == cap[0].to_title_case() {
                input = re
                    .replace(&input, replacement.to_title_case())
                    .parse()
                    .unwrap();
            }
        }
    }

    input
}

#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn to_cyrillic_test() {
        assert_eq!(
            to_cyrillic(String::from("g'ozal G'OZAL G'ozal geliy")),
            String::from("ғозал ҒОЗАЛ Ғозал гелий")
        );
    }

    #[test]
    fn to_latin_test() {
        assert_eq!(
            to_latin(String::from("ғозал ҒОЗАЛ Ғозал гелий")),
            String::from("g‘ozal GʼOZAL Gʼozal geliy")
        );
    }

    #[test]
    fn to_test() {
        assert_eq!(
            to(String::from("ғозал ҒОЗАЛ Ғозал гелий"), "lat"),
            String::from("g‘ozal GʼOZAL Gʼozal geliy")
        );
        assert_eq!(
            to(String::from("g'ozal G'OZAL G'ozal geliy"), "cyr"),
            String::from("ғозал ҒОЗАЛ Ғозал гелий")
        );

        // test TO_LATIN
        assert_eq!(
            to(String::from("қаерда қаёрда ёрда"), "lat"),
            String::from("qayerda qayorda yorda")
        );
        assert_eq!(
            to(
                String::from("13 январ 2021 йил bnuqtai nazar nuqtai nazar"),
                "lat"
            ),
            String::from("13-yanvar 2021-yil bnuqtai nazar nuqtayi nazar")
        );
        assert_eq!(
            to(String::from("аца баер Ша Ë Ёш Ë Чуст Юрт шу гўзал"), "lat"),
            String::from("atsa bayer Sha Yo Yosh Yo Chust Yurt shu go‘zal")
        );

        // test PRE_TO_LATIN
        assert_eq!(
            to(String::from("«дзнъю» \"а\" ДЗНЪЮ"), "lat"),
            String::from("“dznyu” “a” DZNYU")
        );

        //test TO_CYR
        assert_eq!(to(String::from("“a MЎJ"), "cyr"), String::from("«а МЎЪЖ"));

        // test TO_CYR_SUB
        assert_eq!(to(String::from("ʼA Eʼ"), "cyr"), String::from("ЪА ЭЪ"));

        // test TO_CYR_CORRECT
        assert_eq!(
            to(String::from("аE аЯна-даа"), "cyr"),
            String::from("аЭ аЯнадаа")
        );
    }
}
