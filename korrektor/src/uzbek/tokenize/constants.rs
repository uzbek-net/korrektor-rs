pub(super) const LATIN_EXP: [(&str, &str); 7] = [
    ("singil", "si-ngil"),
    ("dengiz", "de-ngiz"),
    ("pešayvon", "pe-shayvon"),
    ("pešona", "pe-shona"),
    ("maishat", "mai-shat"),
    ("išingizni", "ishi-ngiz-ni"),
    ("išingizda", "ishi-ngiz-da"),
];

pub(super) const CYRILLIC_EXP: [(&str, &str); 0] = [];

pub(super) const A_CORRECT: [(&str, &str); 5] = [
    ("g[ʻʼ'‘’‛′ʽ`]", "ğ"),
    ("o[ʻʼ'‘’‛′ʽ`]", "ŏ"),
    ("ʻ|ʼ|'|‘|’|‛|′|ʽ|`", "ʼ"),
    ("sh", "š"),
    ("ch", "č")
];

pub(super) const I_CORRECT: [(&str, &str); 4] = [
    ("ğ", "gʻ"),
    ("ŏ", "o‘"),
    ("š", "sh"),
    ("č", "ch")
];

pub(super) const REPLACE_CYR: [(&str, &str); 2] = [
    ("[аоуэияёюеў]", "V"),
    ("[бвгджзйклмнпрстфхцчшқғҳ]", "C")
];

pub(super) const REPLACE_LAT: [(&str, &str); 2] = [
    ("[aoueiŏ]", "V"),
    ("[bdfghjklmnpqrstvxyzğšč]", "C")
];