// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0429: self cannot appear alone in use

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0429",
    title:       LocalizedText::new(
        "self cannot appear alone as last segment in use",
        "self не может быть последним сегментом в use",
        "use에서 self가 마지막 세그먼트로 단독 사용될 수 없음"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
The self keyword was used alone at the end of a use statement, which is
invalid. The self keyword in use statements is only valid within a
brace-enclosed list of imports.",
        "\
Ключевое слово self использовано в одиночку в конце оператора use,
что недопустимо. Ключевое слово self в операторах use допустимо только
внутри списка импортов в фигурных скобках.",
        "\
use 문 끝에서 self 키워드가 단독으로 사용되었으며, 이는 유효하지 않습니다.
use 문에서 self 키워드는 중괄호로 묶인 임포트 목록 내에서만 유효합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use self within braces",
                "Использовать self внутри фигурных скобок",
                "중괄호 내에서 self 사용"
            ),
            code:        "use std::fmt::{self, Debug};"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Import the namespace directly",
                "Импортировать пространство имён напрямую",
                "네임스페이스 직접 임포트"
            ),
            code:        "use std::fmt; // Instead of use std::fmt::self;"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0429.html"
    }]
};
