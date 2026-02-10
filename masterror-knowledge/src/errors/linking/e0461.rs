// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0461: crate with mismatched target triple

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0461",
    title:       LocalizedText::new(
        "Couldn't find crate with expected target triple",
        "Не удалось найти крейт с ожидаемым целевым триплетом",
        "예상된 타겟 트리플을 가진 크레이트를 찾을 수 없음"
    ),
    category:    Category::Linking,
    explanation: LocalizedText::new(
        "\
Rust cannot find a required crate compiled for the target architecture.
When linking crates together, they must be compiled for the same target
triple (architecture/OS combination). If one crate is compiled for x86_64
and another for ARM, they have incompatible binary formats.",
        "\
Rust не может найти требуемый крейт, скомпилированный для целевой
архитектуры. При связывании крейтов они должны быть скомпилированы
для одного целевого триплета. Если один крейт скомпилирован для x86_64,
а другой для ARM, они имеют несовместимые бинарные форматы.",
        "\
Rust가 타겟 아키텍처용으로 컴파일된 필수 크레이트를 찾을 수 없습니다.
크레이트를 연결할 때 같은 타겟 트리플로 컴파일되어야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use Cargo to manage targets",
                "Использовать Cargo для управления целями",
                "Cargo를 사용하여 타겟 관리"
            ),
            code:        "# Cargo handles target triples automatically"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Recompile with consistent --target flag",
                "Перекомпилировать с одинаковым флагом --target",
                "일관된 --target 플래그로 재컴파일"
            ),
            code:        "rustc --target x86_64-unknown-linux-gnu lib.rs\nrustc --target x86_64-unknown-linux-gnu main.rs"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0461.html"
    }]
};
