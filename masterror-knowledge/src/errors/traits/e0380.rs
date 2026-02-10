// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0380: auto trait declared with method or associated item

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0380",
    title:       LocalizedText::new(
        "Auto trait cannot have methods or associated items",
        "Auto trait не может иметь методы или ассоциированные элементы",
        "auto trait는 메서드나 연관 항목을 가질 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Auto traits cannot have methods or associated items. They are special traits
designed to be automatically implemented for types that meet certain criteria,
and they cannot define any members.

Auto traits like Send and Sync are implemented automatically by the compiler
based on the type's structure.",
        "\
Auto traits не могут иметь методы или ассоциированные элементы. Это специальные
трейты, предназначенные для автоматической реализации для типов, соответствующих
определённым критериям, и они не могут определять никаких членов.

Auto traits как Send и Sync реализуются автоматически компилятором.",
        "\
auto trait는 메서드나 연관 항목을 가질 수 없습니다. 특정 기준을 충족하는 타입에
자동으로 구현되도록 설계된 특별한 트레이트이며, 어떤 멤버도 정의할 수 없습니다.

Send와 Sync 같은 auto trait는 타입의 구조에 따라 컴파일러가 자동으로 구현합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove methods and associated items from auto trait",
                "Удалить методы и элементы из auto trait",
                "auto trait에서 메서드와 연관 항목 제거"
            ),
            code:        "unsafe auto trait MyTrait {\n    // Empty - no methods or associated items allowed\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "RFC 19: Opt-in Builtin Traits",
            url:   "https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0380.html"
        }
    ]
};
