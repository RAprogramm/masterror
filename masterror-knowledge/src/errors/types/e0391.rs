// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0391: type dependency cycle detected

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0391",
    title:       LocalizedText::new(
        "Type dependency cycle detected",
        "Обнаружен цикл зависимости типов",
        "타입 의존성 순환이 감지됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A type dependency cycle has been encountered. This occurs when there is a
circular dependency between types, typically involving trait definitions.

This creates an impossible situation where a type depends on itself (directly
or indirectly) through trait bounds.

Example of cyclic dependency:
- FirstTrait has a supertrait bound on SecondTrait
- SecondTrait has a supertrait bound on FirstTrait
- This creates a circular dependency that cannot be resolved",
        "\
Обнаружен цикл зависимости типов. Это происходит при наличии круговой
зависимости между типами, обычно связанной с определениями трейтов.

Это создаёт невозможную ситуацию, когда тип зависит от себя (прямо или
косвенно) через ограничения трейтов.

Пример циклической зависимости:
- FirstTrait имеет супертрейт SecondTrait
- SecondTrait имеет супертрейт FirstTrait
- Это создаёт круговую зависимость, которую невозможно разрешить",
        "\
타입 의존성 순환이 발견되었습니다. 이는 일반적으로 트레이트 정의를 포함하여
타입 간에 순환 의존성이 있을 때 발생합니다.

이는 타입이 트레이트 바운드를 통해 (직접 또는 간접적으로) 자기 자신에게
의존하는 불가능한 상황을 만듭니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove one of the trait bounds to break the cycle",
                "Удалить одно из ограничений трейта для разрыва цикла",
                "순환을 끊기 위해 트레이트 바운드 중 하나 제거"
            ),
            code:        "trait FirstTrait {\n    // No supertrait bound\n}\n\ntrait SecondTrait : FirstTrait {\n    // Only one direction of dependency\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Restructure trait hierarchy to avoid cycles",
                "Реструктурировать иерархию трейтов для избежания циклов",
                "순환을 피하기 위해 트레이트 계층 구조 재구성"
            ),
            code:        "trait Base {}\ntrait FirstTrait : Base {}\ntrait SecondTrait : Base {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rustc Dev Guide: Queries",
            url:   "https://rustc-dev-guide.rust-lang.org/query.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0391.html"
        }
    ]
};
