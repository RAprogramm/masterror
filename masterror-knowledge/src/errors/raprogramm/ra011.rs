// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA011: Immutability first - prefer self over &mut self

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA011",
    title:        LocalizedText::new(
        "Immutability first: prefer self over &mut self",
        "Сначала неизменяемость: предпочитайте self вместо &mut self",
        "불변성 우선: &mut self보다 self 선호"
    ),
    category:     PracticeCategory::Design,
    explanation:  LocalizedText::new(
        "\
Prefer returning new objects over mutating existing ones. Use `self` instead
of `&mut self` where practical.

Problems with mutable objects:
- Shared state bugs from unexpected modifications
- Thread safety requires synchronization
- Temporal coupling makes operation order matter
- Incomplete state during configuration

Exceptions: Large data structures, I/O, performance-critical loops, Iterator::next",
        "\
Предпочитайте возврат новых объектов вместо изменения существующих.
Используйте `self` вместо `&mut self` где возможно.",
        "\
기존 객체를 변경하는 것보다 새 객체를 반환하는 것을 선호하세요."
    ),
    good_example: r#"Request::new(url)
    .header("Content-Type", "application/json")
    .body(payload)
    .send()"#,
    bad_example:  r#"let mut req = Request::new(url);
req.set_header("Content-Type", "application/json");
req.set_body(payload);
req.send()"#,
    source:       "https://github.com/RAprogramm/RustManifest/blob/main/STRUCTURE.md#7-immutability-first"
};
