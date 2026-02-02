// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Phrase translations for cargo compiler output.
//!
//! Uses Aho-Corasick algorithm for O(n+m) multi-pattern replacement.

use std::sync::LazyLock;

use aho_corasick::AhoCorasick;

use super::Lang;

/// Russian phrase translations (sorted alphabetically by English key).
#[cfg(feature = "lang-ru")]
static PHRASES_RU: &[(&str, &str)] = &[
    ("aborting due to", "прерывание из-за"),
    (
        "as mutable because it is also borrowed as immutable",
        "как изменяемое, т.к. уже заимствовано как неизменяемое"
    ),
    (
        "as mutable more than once at a time",
        "как изменяемое больше одного раза одновременно"
    ),
    (
        "borrow of moved value",
        "заимствование перемещённого значения"
    ),
    (
        "borrowed value does not live long enough",
        "заимствованное значение живёт недостаточно долго"
    ),
    ("cannot borrow", "нельзя заимствовать"),
    ("consider", "рассмотрите"),
    (
        "consider cloning the value if the performance cost is acceptable",
        "рассмотрите клонирование значения, если допустима потеря производительности"
    ),
    ("could not compile", "не удалось скомпилировать"),
    ("does not live long enough", "живёт недостаточно долго"),
    (
        "dropped here while still borrowed",
        "удалено здесь, пока ещё заимствовано"
    ),
    ("due to", "из-за"),
    ("error", "ошибка"),
    ("expected", "ожидается"),
    (
        "expected named lifetime parameter",
        "ожидается именованный параметр времени жизни"
    ),
    ("expected type", "ожидаемый тип"),
    (
        "first borrow later used here",
        "первое заимствование используется здесь"
    ),
    (
        "first mutable borrow occurs here",
        "первое изменяемое заимствование здесь"
    ),
    (
        "for more info about this issue",
        "для информации об этой ошибке"
    ),
    ("found", "найдено"),
    ("found type", "найденный тип"),
    ("has type", "имеет тип"),
    ("help", "подсказка"),
    (
        "immutable borrow later used here",
        "неизменяемое заимствование используется здесь"
    ),
    (
        "immutable borrow occurs here",
        "неизменяемое заимствование здесь"
    ),
    ("mismatched types", "несовпадение типов"),
    (
        "missing lifetime specifier",
        "отсутствует спецификатор времени жизни"
    ),
    ("move occurs because", "перемещение происходит потому что"),
    (
        "mutable borrow occurs here",
        "изменяемое заимствование здесь"
    ),
    ("note", "примечание"),
    ("previous error", "предыдущей ошибки"),
    ("previous errors", "предыдущих ошибок"),
    ("run with", "запустите с"),
    (
        "second mutable borrow occurs here",
        "второе изменяемое заимствование здесь"
    ),
    (
        "this error originates in the macro",
        "эта ошибка возникла в макросе"
    ),
    ("this expression has type", "это выражение имеет тип"),
    (
        "value borrowed here after move",
        "значение заимствовано здесь после перемещения"
    ),
    ("value moved here", "значение перемещено здесь"),
    ("warning", "предупреждение"),
    (
        "which does not implement the `Copy` trait",
        "который не реализует трейт `Copy`"
    )
];

/// Korean phrase translations (sorted alphabetically by English key).
#[cfg(feature = "lang-ko")]
static PHRASES_KO: &[(&str, &str)] = &[
    ("borrow of moved value", "이동된 값의 빌림"),
    ("cannot borrow", "빌릴 수 없습니다"),
    ("error", "에러"),
    ("help", "도움말"),
    ("mismatched types", "타입 불일치"),
    ("note", "참고"),
    ("warning", "경고")
];

/// Pre-built Aho-Corasick automaton for Russian translations.
///
/// Patterns are sorted by length (longest first) to ensure correct replacement
/// when shorter patterns are substrings of longer ones.
#[cfg(feature = "lang-ru")]
static AC_RU: LazyLock<(AhoCorasick, Vec<&'static str>)> = LazyLock::new(|| {
    let mut sorted: Vec<_> = PHRASES_RU.iter().collect();
    sorted.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    let patterns: Vec<_> = sorted.iter().map(|(k, _)| *k).collect();
    let replacements: Vec<_> = sorted.iter().map(|(_, v)| *v).collect();

    let ac = AhoCorasick::new(&patterns).expect("valid patterns");
    (ac, replacements)
});

/// Pre-built Aho-Corasick automaton for Korean translations.
#[cfg(feature = "lang-ko")]
static AC_KO: LazyLock<(AhoCorasick, Vec<&'static str>)> = LazyLock::new(|| {
    let mut sorted: Vec<_> = PHRASES_KO.iter().collect();
    sorted.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    let patterns: Vec<_> = sorted.iter().map(|(k, _)| *k).collect();
    let replacements: Vec<_> = sorted.iter().map(|(_, v)| *v).collect();

    let ac = AhoCorasick::new(&patterns).expect("valid patterns");
    (ac, replacements)
});

/// Translate a phrase to the target language.
///
/// Returns `None` if no translation exists or language is English.
pub fn translate_phrase(phrase: &str, lang: Lang) -> Option<&'static str> {
    let phrases: &[(&str, &str)] = match lang {
        Lang::En => return None,
        #[cfg(feature = "lang-ru")]
        Lang::Ru => PHRASES_RU,
        #[cfg(feature = "lang-ko")]
        Lang::Ko => PHRASES_KO,
        #[allow(unreachable_patterns)]
        _ => return None
    };

    phrases
        .binary_search_by_key(&phrase, |(k, _)| *k)
        .ok()
        .map(|i| phrases[i].1)
}

/// Translate full rendered compiler output.
///
/// Uses pre-built Aho-Corasick automaton for O(n+m) replacement
/// instead of O(n*m) naive string replacement.
pub fn translate_rendered(rendered: &str, lang: Lang) -> String {
    match lang {
        Lang::En => rendered.to_string(),
        #[cfg(feature = "lang-ru")]
        Lang::Ru => {
            let (ac, replacements) = &*AC_RU;
            ac.replace_all(rendered, replacements)
        }
        #[cfg(feature = "lang-ko")]
        Lang::Ko => {
            let (ac, replacements) = &*AC_KO;
            ac.replace_all(rendered, replacements)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "lang-ru")]
    fn test_translate_phrase_ru() {
        assert_eq!(
            translate_phrase("borrow of moved value", Lang::Ru),
            Some("заимствование перемещённого значения")
        );
        assert_eq!(translate_phrase("unknown phrase", Lang::Ru), None);
    }

    #[test]
    fn test_translate_phrase_en() {
        assert_eq!(translate_phrase("borrow of moved value", Lang::En), None);
    }

    #[test]
    #[cfg(feature = "lang-ru")]
    fn test_phrases_sorted() {
        for window in PHRASES_RU.windows(2) {
            assert!(
                window[0].0 < window[1].0,
                "Phrases not sorted: {:?} should come before {:?}",
                window[1].0,
                window[0].0
            );
        }
    }

    #[test]
    #[cfg(feature = "lang-ru")]
    fn test_translate_rendered_ru() {
        let input = "error: borrow of moved value";
        let output = translate_rendered(input, Lang::Ru);
        assert_eq!(output, "ошибка: заимствование перемещённого значения");
    }

    #[test]
    fn test_translate_rendered_en_passthrough() {
        let input = "error: borrow of moved value";
        let output = translate_rendered(input, Lang::En);
        assert_eq!(output, input);
    }
}
