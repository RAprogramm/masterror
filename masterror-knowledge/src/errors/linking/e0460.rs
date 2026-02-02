// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0460: found possibly newer version of crate

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0460",
    title:       LocalizedText::new(
        "Found possibly newer version of crate",
        "Найдена возможно более новая версия крейта",
        "크레이트의 더 새로운 버전이 발견됨"
    ),
    category:    Category::Linking,
    explanation: LocalizedText::new(
        "\
A program depends on two different versions of the same crate, creating an
incompatible dependency chain. This happens when:
- Your crate depends on crate `a` version 1
- Your crate also depends on crate `b`
- Crate `b` depends on crate `a` version 2

The version mismatch is tracked using SVH (Strict Version Hash).",
        "\
Программа зависит от двух разных версий одного и того же крейта,
создавая несовместимую цепочку зависимостей. Это происходит когда:
- Ваш крейт зависит от крейта `a` версии 1
- Ваш крейт также зависит от крейта `b`
- Крейт `b` зависит от крейта `a` версии 2",
        "\
프로그램이 같은 크레이트의 두 가지 다른 버전에 의존하여 호환되지
않는 의존성 체인을 만듭니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use Cargo to manage dependencies",
                "Использовать Cargo для управления зависимостями",
                "Cargo를 사용하여 의존성 관리"
            ),
            code:        "# Cargo automatically resolves dependencies"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Recompile with consistent versions",
                "Перекомпилировать с согласованными версиями",
                "일관된 버전으로 재컴파일"
            ),
            code:        "# Ensure all crates depend on same version"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0460.html"
    }]
};
