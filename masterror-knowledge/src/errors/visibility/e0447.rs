// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0447: pub used inside a function

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0447",
    title:       LocalizedText::new(
        "pub keyword used inside a function",
        "Ключевое слово pub использовано внутри функции",
        "함수 내에서 pub 키워드 사용됨"
    ),
    category:    Category::Visibility,
    explanation: LocalizedText::new(
        "\
The pub (public) keyword was used to mark an item as public inside a
function body. This is invalid because visibility modifiers have no
effect for items defined inside functions.

Items defined inside a function are not accessible from outside that
function's scope regardless of visibility modifiers.

Note: This error is no longer emitted by modern compiler versions.",
        "\
Ключевое слово pub использовано для пометки элемента как публичного
внутри тела функции. Это недопустимо, так как модификаторы видимости
не имеют эффекта для элементов внутри функций.

Примечание: эта ошибка больше не выдаётся современными версиями компилятора.",
        "\
pub(공개) 키워드가 함수 본문 내에서 항목을 공개로 표시하는 데
사용되었습니다. 함수 내에서 정의된 항목에는 가시성 수정자가
효과가 없으므로 유효하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove the pub keyword",
            "Удалить ключевое слово pub",
            "pub 키워드 제거"
        ),
        code:        "fn foo() {\n    struct Bar; // Remove pub\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0447.html"
    }]
};
