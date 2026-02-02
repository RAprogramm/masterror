// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0158: a generic parameter or static has been referenced in a pattern

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0158",
    title:       LocalizedText::new(
        "A generic parameter or static has been referenced in a pattern",
        "Обобщённый параметр или статическая переменная использованы в паттерне",
        "제네릭 매개변수 또는 static이 패턴에서 참조됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A generic parameter or static has been referenced in a pattern match.
The compiler rejects this because it cannot prove that pattern matching
is exhaustive when generic types are involved.

Rust performs type checking on generic methods rather than on monomorphized
instances. Since a trait could have arbitrary implementations, the compiler
cannot guarantee that a pattern will always resolve to a known value.",
        "\
Обобщённый параметр или статическая переменная использованы в паттерне.
Компилятор отклоняет это, потому что не может доказать исчерпывающее
сопоставление паттернов при использовании обобщённых типов.

Rust выполняет проверку типов для обобщённых методов, а не для
мономорфизированных экземпляров.",
        "\
제네릭 매개변수 또는 static이 패턴 매치에서 참조되었습니다.
컴파일러는 제네릭 타입이 관련된 경우 패턴 매칭이 완전하다는 것을
증명할 수 없기 때문에 이를 거부합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use guard clauses instead of direct pattern matching",
            "Использовать условия охраны вместо прямого сопоставления",
            "직접 패턴 매칭 대신 가드 절 사용"
        ),
        code:        "fn test<A: Trait, const Y: char>(arg: char) {\n    match arg {\n        c if c == A::X => println!(\"A::X\"),\n        c if c == Y => println!(\"Y\"),\n        _ => ()\n    }\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Match Expressions",
            url:   "https://doc.rust-lang.org/reference/expressions/match-expr.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0158.html"
        }
    ]
};
