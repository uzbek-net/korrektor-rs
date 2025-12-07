pub(super) const LATIN_EXP: &[(&str, &str)] = &[
    ("singil", "si-ngil"),
    ("dengiz", "de-ngiz"),
    ("pešayvon", "pe-shayvon"),
    ("pešona", "pe-shona"),
    ("maishat", "mai-shat"),
    ("išingizni", "ishi-ngiz-ni"),
    ("išingizda", "ishi-ngiz-da"),
];

pub(super) const CYRILLIC_EXP: &[(&str, &str)] = &[];

pub(super) const A_CORRECT: &[(&str, &str)] = &[
    ("g[ʻʼ'‘’‛′ʽ`]", "ğ"),
    ("o[ʻʼ'‘’‛′ʽ`]", "ŏ"),
    ("ʻ|ʼ|'|‘|’|‛|′|ʽ|`", "ʼ"),
    ("sh", "š"),
    ("ch", "č")
];

pub(super) const I_CORRECT: &[(&str, &str)] = &[
    ("ğ", "gʻ"),
    ("ŏ", "o‘"),
    ("š", "sh"),
    ("č", "ch")
];

pub(super) const REPLACE_CYR: &[(&str, &str)] = &[
    ("[аоуэияёюеў]", "V"),
    ("[бвгджзйклмнпрстфхцчшқғҳ]", "C")
];

pub(super) const REPLACE_LAT: &[(&str, &str)] = &[
    ("[aoueiŏ]", "V"),
    ("[bdfghjklmnpqrstvxyzğšč]", "C")
];