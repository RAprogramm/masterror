// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0109: type arguments not allowed for this type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0109",
    title:       LocalizedText::new(
        "Type arguments not allowed for this type",
        "Аргументы типа не разрешены для этого типа",
        "이 타입에는 타입 인수가 허용되지 않음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
You tried to provide a generic argument to a type which doesn't need it.
Primitive types like u32, bool, i64 don't accept type parameters.

Generic arguments for enum variant constructors go after the variant,
not after the enum. For example, write Option::None::<u32> rather than
Option::<u32>::None.",
        "\
Вы попытались передать обобщённый аргумент типу, который его не принимает.
Примитивные типы вроде u32, bool, i64 не принимают параметры типа.

Обобщённые аргументы для конструкторов вариантов перечислений указываются
после варианта, а не после перечисления.",
        "\
제네릭 인수를 받지 않는 타입에 제네릭 인수를 제공하려고 했습니다.
u32, bool, i64와 같은 기본 타입은 타입 매개변수를 받지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove the type argument from primitive type",
                "Удалить аргумент типа у примитивного типа",
                "기본 타입에서 타입 인수 제거"
            ),
            code:        "type X = u32; // not u32<i32>"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Place generic args after enum variant",
                "Поместить обобщённые аргументы после варианта",
                "열거형 변형 뒤에 제네릭 인수 배치"
            ),
            code:        "Option::None::<u32> // not Option::<u32>::None"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Types",
            url:   "https://doc.rust-lang.org/reference/types.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0109.html"
        }
    ]
};
