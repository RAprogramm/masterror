// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA016: Minimize unsafe blocks

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA016",
    title:        LocalizedText::new(
        "Minimize unsafe blocks",
        "Минимизируйте unsafe блоки",
        "unsafe 블록 최소화"
    ),
    category:     PracticeCategory::Safety,
    explanation:  LocalizedText::new(
        "\
Unsafe code bypasses Rust's safety guarantees. Every unsafe block is a potential
source of undefined behavior, memory corruption, or security vulnerabilities.

Isolate unsafe code in small, well-documented functions with safe wrappers.
Prove invariants with comments and tests. Consider using safe abstractions
from crates like `zerocopy`, `bytemuck`, or standard library equivalents.",
        "\
Unsafe код обходит гарантии безопасности Rust. Каждый unsafe блок —
потенциальный источник неопределённого поведения, повреждения памяти
или уязвимостей безопасности.

Изолируйте unsafe код в небольших, хорошо документированных функциях
с безопасными обёртками. Докажите инварианты комментариями и тестами.",
        "\
unsafe 코드는 Rust의 안전 보장을 우회합니다. 모든 unsafe 블록은
정의되지 않은 동작, 메모리 손상 또는 보안 취약점의 잠재적 원인입니다."
    ),
    good_example: r#"/// SAFETY: `ptr` must be valid and properly aligned.
/// The caller ensures the pointer comes from a valid allocation.
unsafe fn read_value(ptr: *const u32) -> u32 {
    *ptr
}

// Safe wrapper
pub fn get_value(slice: &[u32], index: usize) -> Option<u32> {
    slice.get(index).copied()
}"#,
    bad_example:  r#"fn process(data: &[u8]) {
    unsafe {
        // 100 lines of unsafe code with no invariant documentation
        let ptr = data.as_ptr();
        // ...
    }
}"#,
    source:       "https://github.com/RAprogramm/RustManifest#memory-safety"
};
