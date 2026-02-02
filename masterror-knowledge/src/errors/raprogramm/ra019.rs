// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA019: Avoid transmute

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA019",
    title:        LocalizedText::new(
        "Avoid std::mem::transmute",
        "Избегайте std::mem::transmute",
        "std::mem::transmute 피하기"
    ),
    category:     PracticeCategory::Safety,
    explanation:  LocalizedText::new(
        "\
`transmute` reinterprets bits of one type as another without any checks.
It can easily cause undefined behavior if types have different layouts,
alignment requirements, or validity invariants.

Prefer safe alternatives:
- `as` casts for numeric conversions
- `From`/`Into` traits for type conversions
- `bytemuck` crate for safe transmutes between Pod types
- `zerocopy` crate for zero-copy parsing",
        "\
`transmute` переинтерпретирует биты одного типа как другой без проверок.
Это легко может вызвать неопределённое поведение, если типы имеют разные
layouts, требования выравнивания или инварианты валидности.

Используйте безопасные альтернативы:
- `as` для числовых преобразований
- `From`/`Into` трейты для преобразования типов
- `bytemuck` крейт для безопасных transmute между Pod типами",
        "\
`transmute`는 검사 없이 한 타입의 비트를 다른 타입으로 재해석합니다.
타입이 다른 레이아웃, 정렬 요구 사항 또는 유효성 불변성을 가지면
정의되지 않은 동작을 쉽게 유발할 수 있습니다."
    ),
    good_example: r#"// Safe numeric conversion
let x: u32 = 42;
let y: i32 = x as i32;

// Safe bytes conversion with bytemuck
use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Pixel { r: u8, g: u8, b: u8, a: u8 }

let bytes: [u8; 4] = [255, 0, 0, 255];
let pixel: Pixel = bytemuck::cast(bytes);"#,
    bad_example:  r#"let x: u32 = 42;
let y: f32 = unsafe { std::mem::transmute(x) };

// Even worse - different sizes!
let s: &str = "hello";
let n: usize = unsafe { std::mem::transmute(s) }; // UB!"#,
    source:       "https://github.com/RAprogramm/RustManifest#memory-safety"
};
