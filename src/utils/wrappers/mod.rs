use crate::utils::pcre as ipcre;
use pcre::Pcre;

// wrapping special text with 〈〉 to avoid correction and transliteration
pub fn get_wrapped_text(text: &str) -> String {
    let mut result = wrap_mails(text);
    result = wrap_urls(&result);
    result = wrap_usernames(&result);

    result
}

fn wrap_mails(text: &str) -> String {
    let mut re: Pcre = Pcre::compile(
        "([\\w-]+(?:\\.[\\w-]+)*)@((?:[\\w-]+\\.)*\\w[\\w-]{0,66})\\.([a-z]{2,6}(?:\\.[a-z]{2})?)",
    )
    .unwrap();
    let matches = re.matches(text);

    ipcre::wrap_matches(text, matches)
}

fn wrap_urls(text: &str) -> String {
    let mut re: Pcre = Pcre::compile("(?i)\\b(?:(?:https?|ftp|file|ssh):\\/\\/|www\\.|ftp\\.)[-A-Z0-9+&@#\\/%=~_|$?!:,.]*[A-Z0-9+&@#\\/%=~_|$]").unwrap();
    let matches = re.matches(text);

    ipcre::wrap_matches(text, matches)
}

fn wrap_usernames(text: &str) -> String {
    let mut re: Pcre = Pcre::compile(r"([\w-]+(?:\.[\w-]+)*)@((?:[\w-]+\.)*\w[\w-]{0,66})\.([a-z]{2,6}(?:\.[a-z]{2})?)|@(?!.*\-|.*\-$)[a-zA-Z0-9][\w-]+[a-zA-Z0-9]{0,39}").unwrap();
    let matches = re.matches(text);

    ipcre::wrap_matches(text, matches)
}

#[cfg(test)]
pub mod as_tests {
    use super::*;

    #[test]
    fn wrap_mails_test() {
        assert_eq!(
            wrap_mails(
                "@ki-d @ki- @hello nyan@mail.uz nya@mail.uz https://nyan.com go'zal @crystalny"
            ),
            "@ki-d @ki- @hello 〈nyan@mail.uz〉 〈nya@mail.uz〉 https://nyan.com go'zal @crystalny"
                .to_string()
        );
    }

    #[test]
    fn wrap_urls_test() {
        assert_eq!(wrap_urls("@ki-d @ki- @hello 〈nyan@mail.uz〉 〈nya@mail.uz〉 https://nyan.com go'zal @crystalny"),
                   "@ki-d @ki- @hello 〈nyan@mail.uz〉 〈nya@mail.uz〉 〈https://nyan.com〉 go'zal @crystalny".to_string());
    }

    #[test]
    fn wrap_usernames_test() {
        assert_eq!(wrap_usernames("@ki-d @ki- @hello 〈nyan@mail.uz〉 〈nya@mail.uz〉 〈https://nyan.com〉 go'zal @crystalny"),
                   "@ki-d @ki- 〈@hello〉 〈〈nyan@mail.uz〉〉 〈〈nya@mail.uz〉〉 〈https://nyan.com〉 go'zal 〈@crystalny〉".to_string());
    }

    #[test]
    fn get_wrapped_text_test() {
        assert_eq!(get_wrapped_text("@ki-d @ki- @hello nyan@mail.uz nya@mail.uz https://nyan.com go'zal @crystalny"),
        String::from("@ki-d @ki- 〈@hello〉 〈〈nyan@mail.uz〉〉 〈〈nya@mail.uz〉〉 〈https://nyan.com〉 go'zal 〈@crystalny〉"));
    }
}
