// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0393: type parameter with Self default not specified

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0393",
    title:       LocalizedText::new(
        "Type parameter referencing Self must be specified",
        "Параметр типа со ссылкой на Self должен быть указан",
        "Self를 참조하는 타입 매개변수를 지정해야 함"
    ),
    category:    Category::Generics,
    explanation: LocalizedText::new(
        "\
A type parameter which references `Self` in its default value was not specified.
This error occurs when a trait has a default type parameter that references
`Self`, but the trait is used as a trait object without explicitly specifying
that type parameter.

Trait objects require a single, fully-defined trait. When a default parameter
is `Self`, the trait effectively changes for each concrete type:
- i32 would need to implement A<i32>
- bool would need to implement A<bool>

Since each type implements a different version of the trait, they cannot be
unified into a single trait object.",
        "\
Параметр типа со ссылкой на `Self` в значении по умолчанию не был указан.
Эта ошибка возникает, когда трейт имеет параметр типа по умолчанию, ссылающийся
на `Self`, но трейт используется как трейт-объект без явного указания этого
параметра типа.

Трейт-объекты требуют единого, полностью определённого трейта. Когда параметр
по умолчанию - `Self`, трейт фактически меняется для каждого конкретного типа:
- i32 должен реализовывать A<i32>
- bool должен реализовывать A<bool>

Поскольку каждый тип реализует разную версию трейта, они не могут быть
объединены в единый трейт-объект.",
        "\
기본값에서 `Self`를 참조하는 타입 매개변수가 지정되지 않았습니다.
이 오류는 트레이트가 `Self`를 참조하는 기본 타입 매개변수를 가지고 있지만,
해당 타입 매개변수를 명시적으로 지정하지 않고 트레이트 객체로 사용될 때 발생합니다.

트레이트 객체는 단일하고 완전히 정의된 트레이트를 필요로 합니다.
기본 매개변수가 `Self`일 때, 각 구체적인 타입에 대해 트레이트가 효과적으로 변경됩니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Explicitly specify the concrete type parameter",
            "Явно указать конкретный параметр типа",
            "구체적인 타입 매개변수를 명시적으로 지정"
        ),
        code:        "trait A<T = Self> {}\n\nfn together_we_will_rule_the_galaxy(son: &dyn A<i32>) {} // Ok!"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Trait Objects",
            url:   "https://doc.rust-lang.org/reference/types/trait-object.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0393.html"
        }
    ]
};
