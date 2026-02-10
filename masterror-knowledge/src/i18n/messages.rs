// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! UI messages for masterror-cli.

use crate::define_messages;

define_messages! {
    pub enum UiMsg {
        LabelTranslation {
            en: "Translation:",
            ru: "Перевод:",
            ko: "번역:",
        }
        LabelWhy {
            en: "Why this happens:",
            ru: "Почему это происходит:",
            ko: "왜 이런 일이 발생하나요:",
        }
        LabelFix {
            en: "How to fix:",
            ru: "Как исправить:",
            ko: "해결 방법:",
        }
        LabelLink {
            en: "Learn more:",
            ru: "Подробнее:",
            ko: "더 알아보기:",
        }
        LabelWhyMatters {
            en: "Why this matters:",
            ru: "Почему это важно:",
            ko: "왜 중요한가:",
        }
        LabelHowToApply {
            en: "How to apply:",
            ru: "Как применять:",
            ko: "적용 방법:",
        }
        LabelAvoid {
            en: "Avoid",
            ru: "Избегайте",
            ko: "피하세요",
        }
        LabelPrefer {
            en: "Prefer",
            ru: "Предпочитайте",
            ko: "선호하세요",
        }

        CategoryOwnership {
            en: "Ownership",
            ru: "Владение",
            ko: "소유권",
        }
        CategoryBorrowing {
            en: "Borrowing",
            ru: "Заимствование",
            ko: "빌림",
        }
        CategoryLifetimes {
            en: "Lifetimes",
            ru: "Времена жизни",
            ko: "라이프타임",
        }
        CategoryTypes {
            en: "Types",
            ru: "Типы",
            ko: "타입",
        }
        CategoryTraits {
            en: "Traits",
            ru: "Трейты",
            ko: "트레이트",
        }
        CategoryResolution {
            en: "Name Resolution",
            ru: "Разрешение имён",
            ko: "이름 확인",
        }

        CategoryErrorHandling {
            en: "Error Handling",
            ru: "Обработка ошибок",
            ko: "에러 처리",
        }
        CategoryPerformance {
            en: "Performance",
            ru: "Производительность",
            ko: "성능",
        }
        CategoryNaming {
            en: "Naming",
            ru: "Именование",
            ko: "명명",
        }
        CategoryDocumentation {
            en: "Documentation",
            ru: "Документация",
            ko: "문서화",
        }
        CategoryDesign {
            en: "Design",
            ru: "Дизайн",
            ko: "설계",
        }
        CategoryTesting {
            en: "Testing",
            ru: "Тестирование",
            ko: "테스트",
        }
        CategorySecurity {
            en: "Security",
            ru: "Безопасность",
            ko: "보안",
        }

        UnknownCode {
            en: "Unknown code",
            ru: "Неизвестный код",
            ko: "알 수 없는 코드",
        }
        Category {
            en: "Category",
            ru: "Категория",
            ko: "카테고리",
        }

        InitTitle {
            en: "masterror configuration",
            ru: "Настройка masterror",
            ko: "masterror 설정",
        }
        InitSuccess {
            en: "Configuration saved to",
            ru: "Конфигурация сохранена в",
            ko: "설정이 저장됨:",
        }
        InitLangPrompt {
            en: "Language",
            ru: "Язык",
            ko: "언어",
        }
        InitColorPrompt {
            en: "Colored output",
            ru: "Цветной вывод",
            ko: "색상 출력",
        }
        InitDisplayPrompt {
            en: "Display sections:",
            ru: "Секции для отображения:",
            ko: "표시 섹션:",
        }
        InitShowTranslation {
            en: "Show translation",
            ru: "Показывать перевод",
            ko: "번역 표시",
        }
        InitShowWhy {
            en: "Show explanation",
            ru: "Показывать объяснение",
            ko: "설명 표시",
        }
        InitShowFix {
            en: "Show fix suggestions",
            ru: "Показывать исправления",
            ko: "수정 제안 표시",
        }
        InitShowLinks {
            en: "Show documentation links",
            ru: "Показывать ссылки",
            ko: "문서 링크 표시",
        }
        InitShowOriginal {
            en: "Show original compiler output",
            ru: "Показывать оригинальный вывод",
            ko: "원본 컴파일러 출력 표시",
        }
        InitSavePrompt {
            en: "Where to save configuration?",
            ru: "Где сохранить настройки?",
            ko: "설정을 어디에 저장할까요?",
        }
        InitSaveGlobal {
            en: "Global (~/.config/masterror/) - applies to all projects",
            ru: "Глобально (~/.config/masterror/) - для всех проектов",
            ko: "전역 (~/.config/masterror/) - 모든 프로젝트에 적용",
        }
        InitSaveLocal {
            en: "Local (.masterror.toml) - only this project",
            ru: "Локально (.masterror.toml) - только этот проект",
            ko: "로컬 (.masterror.toml) - 이 프로젝트만",
        }
        InitTip {
            en: "You can change settings anytime by editing:",
            ru: "Вы можете изменить настройки в любой момент, отредактировав:",
            ko: "언제든지 다음 파일을 편집하여 설정을 변경할 수 있습니다:",
        }
        InitUsage {
            en: "Start using masterror:",
            ru: "Начните использовать masterror:",
            ko: "masterror 사용 시작:",
        }
    }
}
