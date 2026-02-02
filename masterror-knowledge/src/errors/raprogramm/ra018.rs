// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA018: Avoid mutable statics

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA018",
    title:        LocalizedText::new(
        "Avoid mutable statics",
        "Избегайте изменяемых static переменных",
        "변경 가능한 static 변수 피하기"
    ),
    category:     PracticeCategory::Safety,
    explanation:  LocalizedText::new(
        "\
Mutable statics (`static mut`) are inherently unsafe and a common source of
data races in concurrent code. Any access requires an unsafe block.

Use thread-safe alternatives:
- `std::sync::OnceLock` for lazy initialization
- `std::sync::Mutex` or `RwLock` for mutable shared state
- `std::sync::atomic` types for simple counters/flags
- `thread_local!` for thread-local storage",
        "\
Изменяемые static (`static mut`) по своей природе небезопасны и являются
частым источником гонок данных в многопоточном коде.

Используйте потокобезопасные альтернативы:
- `std::sync::OnceLock` для ленивой инициализации
- `std::sync::Mutex` или `RwLock` для изменяемого общего состояния
- `std::sync::atomic` типы для простых счётчиков/флагов",
        "\
변경 가능한 static(`static mut`)은 본질적으로 안전하지 않으며
동시성 코드에서 데이터 경쟁의 일반적인 원인입니다."
    ),
    good_example: r#"use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| Config::load())
}"#,
    bad_example:  r#"static mut COUNTER: u32 = 0;

fn increment() {
    unsafe {
        COUNTER += 1; // Data race!
    }
}"#,
    source:       "https://github.com/RAprogramm/RustManifest#memory-safety"
};
