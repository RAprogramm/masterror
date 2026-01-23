// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Korean locale.

use std::collections::HashMap;

pub fn messages() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    // Labels
    m.insert("label-translation", "번역:");
    m.insert("label-why", "원인:");
    m.insert("label-fix", "해결:");
    m.insert("label-link", "링크:");

    // Categories
    m.insert("category-ownership", "소유권");
    m.insert("category-types", "타입");
    m.insert("category-lifetimes", "라이프타임");
    m.insert("category-borrowing", "빌림");

    // E0382 - Use of moved value
    m.insert("e0382-title", "이동된 값 사용");
    m.insert("e0382-translation", "이동된 값의 빌림");
    m.insert(
        "e0382-explanation",
        "\
Rust에서 각 값은 정확히 하나의 소유자를 가집니다. 이것이 가비지 컬렉터 없이
메모리 안전성을 보장하는 기반입니다.

값을 다른 변수에 할당하거나 함수에 전달하면 소유권이 새 위치로 이동합니다.
원래 변수는 무효화되어 더 이상 사용할 수 없습니다.

이는 Rust가 메모리를 언제 해제해야 하는지 정확히 알아야 하기 때문입니다.
소유자가 하나이면 정리 책임이 명확합니다."
    );
    m.insert("e0382-fix-clone-desc", "값을 복제 (깊은 복사)");
    m.insert("e0382-fix-borrow-desc", "참조로 빌림 (복사 없음)");
    m.insert("e0382-fix-copy-desc", "Copy 트레이트 구현 (작은 타입용)");

    // E0502 - Cannot borrow as mutable (already borrowed as immutable)
    m.insert("e0502-title", "가변으로 빌릴 수 없음 (이미 불변으로 빌림)");
    m.insert(
        "e0502-translation",
        "이미 불변으로 빌려서 가변으로 빌릴 수 없음"
    );
    m.insert(
        "e0502-explanation",
        "\
Rust는 엄격한 빌림 규칙을 적용합니다: 하나의 가변 참조 또는 여러 불변 참조를
가질 수 있지만, 동시에 둘 다 가질 수는 없습니다.

이는 컴파일 시 데이터 경쟁을 방지합니다. 다른 사람이 읽는 동안 데이터를
변경할 수 있다면, 읽는 사람은 일관성 없는 상태를 볼 수 있습니다.

불변 빌림은 코드 뒤에서 사용되기 때문에 여전히 \"활성\" 상태입니다."
    );
    m.insert("e0502-fix-scope-desc", "변경 전에 불변 빌림 종료");
    m.insert("e0502-fix-clone-desc", "변경 전에 데이터 복제");

    // E0499 - Cannot borrow as mutable more than once
    m.insert("e0499-title", "가변으로 두 번 이상 빌릴 수 없음");
    m.insert(
        "e0499-translation",
        "동시에 가변으로 두 번 이상 빌릴 수 없음"
    );
    m.insert(
        "e0499-explanation",
        "\
Rust는 데이터에 대해 한 번에 하나의 가변 참조만 허용합니다.
이는 불변 빌림 규칙보다 엄격하며 모든 별명 변경을 방지합니다.

왜? 동일한 데이터에 대한 두 개의 가변 참조는 다음을 초래할 수 있습니다:
- 동시 코드에서 데이터 경쟁
- 반복자 무효화
- 재할당 후 댕글링 포인터

이 규칙은 컴파일 시 확인되어 fearless concurrency를 제공합니다."
    );
    m.insert("e0499-fix-scope-desc", "스코프를 사용하여 빌림 수명 제한");
    m.insert(
        "e0499-fix-refcell-desc",
        "내부 가변성을 위해 RefCell 사용 (런타임 검사)"
    );

    // E0308 - Mismatched types
    m.insert("e0308-title", "타입 불일치");
    m.insert("e0308-translation", "타입 불일치");
    m.insert(
        "e0308-explanation",
        "\
Rust는 정적 타입 언어이며 암시적 타입 변환을 수행하지 않습니다.
모든 값에는 특정 타입이 있으며 컴파일러가 타입 일관성을 보장합니다.

이는 다른 언어에서 런타임 오류가 될 버그를 컴파일 시 잡습니다.
타입 시스템은 장애물이 아니라 친구입니다."
    );
    m.insert(
        "e0308-fix-convert-desc",
        "문자열을 숫자로 변환하려면 parse() 사용"
    );
    m.insert("e0308-fix-as-desc", "숫자 타입 캐스팅에 'as' 사용");

    // E0106 - Missing lifetime specifier
    m.insert("e0106-title", "라이프타임 지정자 누락");
    m.insert("e0106-translation", "라이프타임 지정자 누락");
    m.insert(
        "e0106-explanation",
        "\
Rust의 참조에는 라이프타임이 있습니다 - 참조가 얼마나 오래 유효한지 설명합니다.
보통 컴파일러가 라이프타임을 추론하지만, 때로는 명시해야 합니다.

라이프타임 어노테이션은 값이 얼마나 오래 사는지 변경하지 않습니다.
컴파일러가 안전성을 확인할 수 있도록 참조 간의 관계를 설명합니다."
    );
    m.insert("e0106-fix-lifetime-desc", "명시적 라이프타임 매개변수 추가");
    m.insert("e0106-fix-owned-desc", "참조 대신 소유 타입 사용");

    // E0597 - Value does not live long enough
    m.insert("e0597-title", "값이 충분히 오래 살지 않음");
    m.insert("e0597-translation", "값이 충분히 오래 살지 않음");
    m.insert(
        "e0597-explanation",
        "\
참조가 사용되기 전에 파괴될 것에 대한 참조를 만들고 있습니다.
이는 댕글링 포인터를 생성합니다.

Rust는 컴파일 시 이를 방지합니다. 참조되는 값은 적어도 참조 자체만큼
오래 살아야 합니다."
    );
    m.insert("e0597-fix-move-desc", "값을 외부 스코프로 이동");
    m.insert("e0597-fix-owned-desc", "참조 대신 소유 값 반환");

    m
}

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    // Error headers
    m.insert("error", "오류");
    m.insert("warning", "경고");
    m.insert("note", "참고");
    m.insert("help", "도움말");

    // E0382 - borrow of moved value
    m.insert("borrow of moved value", "이동된 값의 빌림");
    m.insert("move occurs because", "이동이 발생하는 이유:");
    m.insert(
        "which does not implement the `Copy` trait",
        "`Copy` 트레이트를 구현하지 않음"
    );
    m.insert("value moved here", "값이 여기서 이동됨");
    m.insert(
        "value borrowed here after move",
        "이동 후 여기서 값이 빌림됨"
    );
    m.insert(
        "consider cloning the value if the performance cost is acceptable",
        "성능 비용이 허용되면 값을 복제하는 것을 고려하세요"
    );

    // E0502 - cannot borrow as mutable
    m.insert("cannot borrow", "빌릴 수 없음");
    m.insert(
        "as mutable because it is also borrowed as immutable",
        "가변으로 (이미 불변으로 빌림)"
    );
    m.insert("immutable borrow occurs here", "불변 빌림이 여기서 발생");
    m.insert("mutable borrow occurs here", "가변 빌림이 여기서 발생");
    m.insert(
        "immutable borrow later used here",
        "불변 빌림이 여기서 나중에 사용됨"
    );

    // E0499 - cannot borrow as mutable more than once
    m.insert(
        "as mutable more than once at a time",
        "동시에 가변으로 두 번 이상"
    );
    m.insert(
        "first mutable borrow occurs here",
        "첫 번째 가변 빌림이 여기서 발생"
    );
    m.insert(
        "second mutable borrow occurs here",
        "두 번째 가변 빌림이 여기서 발생"
    );
    m.insert(
        "first borrow later used here",
        "첫 번째 빌림이 여기서 나중에 사용됨"
    );

    // E0308 - mismatched types
    m.insert("mismatched types", "타입 불일치");
    m.insert("expected", "예상");
    m.insert("found", "발견");
    m.insert("expected type", "예상 타입");
    m.insert("found type", "발견된 타입");
    m.insert("this expression has type", "이 표현식의 타입:");

    // E0106 - missing lifetime
    m.insert("missing lifetime specifier", "라이프타임 지정자 누락");
    m.insert(
        "expected named lifetime parameter",
        "명명된 라이프타임 매개변수 예상"
    );

    // E0597 - does not live long enough
    m.insert("does not live long enough", "충분히 오래 살지 않음");
    m.insert(
        "borrowed value does not live long enough",
        "빌린 값이 충분히 오래 살지 않음"
    );
    m.insert(
        "dropped here while still borrowed",
        "아직 빌린 상태에서 여기서 삭제됨"
    );

    // Common phrases
    m.insert("has type", "타입을 가짐");
    m.insert("consider", "고려하세요");
    m.insert(
        "this error originates in the macro",
        "이 오류는 매크로에서 발생"
    );
    m.insert("run with", "실행:");
    m.insert(
        "for more info about this issue",
        "이 문제에 대한 자세한 정보"
    );
    m.insert("aborting due to", "중단 원인:");
    m.insert("previous error", "이전 오류");
    m.insert("previous errors", "이전 오류들");
    m.insert("could not compile", "컴파일 실패");
    m.insert("due to", "원인:");

    m
}
