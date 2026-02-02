// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0463: can't find crate

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0463",
    title:       LocalizedText::new(
        "Can't find crate",
        "Не удаётся найти крейт",
        "크레이트를 찾을 수 없음"
    ),
    category:    Category::Linking,
    explanation: LocalizedText::new(
        "\
A crate was declared but cannot be found. This happens when:
- The crate is not present in dependencies
- The crate exists under a different name
- For std/core: cross-compiling for an unsupported target

If missing std or core when cross-compiling:
- Add precompiled version via rustup target add
- Build std from source with cargo build -Z build-std
- Use #![no_std] to avoid requiring std",
        "\
Крейт объявлен, но не найден. Это происходит когда:
- Крейт отсутствует в зависимостях
- Крейт существует под другим именем
- Для std/core: кросс-компиляция для неподдерживаемой цели

Если отсутствует std или core при кросс-компиляции:
- Добавить прекомпилированную версию через rustup target add
- Собрать std из исходников с cargo build -Z build-std
- Использовать #![no_std]",
        "\
크레이트가 선언되었지만 찾을 수 없습니다. 다음 경우에 발생합니다:
- 크레이트가 의존성에 없음
- 크레이트가 다른 이름으로 존재
- std/core의 경우: 지원되지 않는 타겟으로 크로스 컴파일"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add the crate to Cargo.toml",
                "Добавить крейт в Cargo.toml",
                "Cargo.toml에 크레이트 추가"
            ),
            code:        "[dependencies]\nfoo = \"1.0\""
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Add target for cross-compilation",
                "Добавить цель для кросс-компиляции",
                "크로스 컴파일을 위한 타겟 추가"
            ),
            code:        "rustup target add thumbv7em-none-eabihf"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0463.html"
    }]
};
