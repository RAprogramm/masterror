// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0455: platform-specific link kind

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0455",
    title:       LocalizedText::new(
        "Platform-specific link kind not supported on this target",
        "Специфичный для платформы тип ссылки не поддерживается",
        "이 타겟에서 플랫폼별 링크 종류가 지원되지 않음"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
Some linking kinds are target-specific and not supported on all platforms:
- kind=framework: only supported on macOS
- kind=raw-dylib: only supported on Windows-like platforms

Using these on unsupported platforms will cause this error.",
        "\
Некоторые типы связывания специфичны для платформы:
- kind=framework: поддерживается только на macOS
- kind=raw-dylib: поддерживается только на Windows-подобных платформах

Использование их на неподдерживаемых платформах вызовет эту ошибку.",
        "\
일부 링크 종류는 플랫폼별로 특정됩니다:
- kind=framework: macOS에서만 지원
- kind=raw-dylib: Windows 유사 플랫폼에서만 지원"
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use conditional compilation",
            "Использовать условную компиляцию",
            "조건부 컴파일 사용"
        ),
        code:        "#[cfg_attr(target_os = \"macos\", link(name = \"CoreServices\", kind = \"framework\"))]\nextern \"C\" {}"
    }],
    links:       &[
        DocLink {
            title: "Conditional Compilation",
            url:   "https://doc.rust-lang.org/reference/attributes.html#conditional-compilation"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0455.html"
        }
    ]
};
