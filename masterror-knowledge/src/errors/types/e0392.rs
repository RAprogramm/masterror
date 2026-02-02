// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0392: unused type or lifetime parameter

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0392",
    title:       LocalizedText::new(
        "Unused type or lifetime parameter",
        "Неиспользуемый параметр типа или времени жизни",
        "사용되지 않는 타입 또는 라이프타임 매개변수"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A type or lifetime parameter has been declared but is not actually used.
The compiler requires that all declared generic parameters be utilized in
the definition.

If you need to specify a lifetime constraint with raw pointers without
storing actual data, use PhantomData to tell the compiler about the
relationship.",
        "\
Параметр типа или времени жизни объявлен, но фактически не используется.
Компилятор требует, чтобы все объявленные параметры были использованы.

Если нужно указать ограничение времени жизни с сырыми указателями без
хранения данных, используйте PhantomData.",
        "\
타입 또는 라이프타임 매개변수가 선언되었지만 실제로 사용되지 않습니다.
컴파일러는 선언된 모든 제네릭 매개변수가 정의에서 사용되어야 합니다.

원시 포인터와 함께 라이프타임 제약을 지정해야 하지만 실제 데이터를
저장하지 않는 경우 PhantomData를 사용하세요."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove unused parameter",
                "Удалить неиспользуемый параметр",
                "사용되지 않는 매개변수 제거"
            ),
            code:        "enum Foo {\n    Bar,\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use the parameter in the type",
                "Использовать параметр в типе",
                "타입에서 매개변수 사용"
            ),
            code:        "enum Foo<T> {\n    Bar(T),\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use PhantomData for lifetime constraints",
                "Использовать PhantomData для ограничений времени жизни",
                "라이프타임 제약에 PhantomData 사용"
            ),
            code:        "use std::marker::PhantomData;\n\nstruct Foo<'a, T: 'a> {\n    x: *const T,\n    phantom: PhantomData<&'a T>\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust std::marker::PhantomData",
            url:   "https://doc.rust-lang.org/std/marker/struct.PhantomData.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0392.html"
        }
    ]
};
