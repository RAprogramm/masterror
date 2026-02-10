// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0801: invalid generic receiver type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0801",
    title:       LocalizedText::new(
        "Invalid generic receiver type",
        "Недопустимый обобщённый тип получателя",
        "잘못된 제네릭 수신자 타입"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The `self` parameter in a method has an invalid generic receiver type.

Methods take a special first parameter called `self`. With arbitrary self types,
you can use types that implement `Deref<Target=Self>` like `Rc<Self>` or `Box<Self>`.

However, the `self` type must be CONCRETE. Generic `self` types are not permitted.
A `self` type will be rejected if it is a type parameter defined on the method.

Invalid: `fn foo<R: Deref<Target=Self>>(self: R)`
Invalid: `fn foo(self: impl Deref<Target=Self>)`
Valid: `fn foo(self: Rc<Self>)`",
        "\
Параметр `self` в методе имеет недопустимый обобщённый тип получателя.

Методы принимают специальный первый параметр `self`. С произвольными типами self
можно использовать типы, реализующие `Deref<Target=Self>`, такие как `Rc<Self>`.

Однако тип `self` должен быть КОНКРЕТНЫМ. Обобщённые типы `self` не разрешены.
Тип `self` будет отклонён, если он является параметром типа, определённым в методе.

Неверно: `fn foo<R: Deref<Target=Self>>(self: R)`
Верно: `fn foo(self: Rc<Self>)`",
        "\
메서드의 `self` 매개변수가 잘못된 제네릭 수신자 타입을 가지고 있습니다.
`self` 타입은 반드시 구체적이어야 합니다. 제네릭 `self` 타입은 허용되지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use concrete self type",
            "Использовать конкретный тип self",
            "구체적인 self 타입 사용"
        ),
        code:        "\
use std::rc::Rc;

impl Foo {
    fn foo(self: Rc<Self>) {}
}"
    }],
    links:       &[
        DocLink {
            title: "Arbitrary Self Types",
            url:   "https://doc.rust-lang.org/unstable-book/language-features/arbitrary-self-types.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0801.html"
        }
    ]
};
