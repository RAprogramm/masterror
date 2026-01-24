// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Internationalization system for masterror-cli.
//!
//! Provides compile-time localization with zero runtime allocation.

pub mod messages;
pub mod phrases;

/// Supported languages.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Lang {
    /// English (default).
    #[default]
    En = 0,
    /// Russian.
    #[cfg(feature = "lang-ru")]
    Ru = 1,
    /// Korean.
    #[cfg(feature = "lang-ko")]
    Ko = 2
}

impl Lang {
    /// Parse language from string, fallback to English.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::i18n::Lang;
    ///
    /// assert_eq!(Lang::from_code("ru"), Lang::Ru);
    /// assert_eq!(Lang::from_code("unknown"), Lang::En);
    /// ```
    pub fn from_code(s: &str) -> Self {
        match s {
            #[cfg(feature = "lang-ru")]
            "ru" | "RU" | "Ru" => Self::Ru,
            #[cfg(feature = "lang-ko")]
            "ko" | "KO" | "Ko" => Self::Ko,
            _ => Self::En
        }
    }

    /// Get language code as string.
    pub const fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            #[cfg(feature = "lang-ru")]
            Self::Ru => "ru",
            #[cfg(feature = "lang-ko")]
            Self::Ko => "ko"
        }
    }

    /// Get language display name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            #[cfg(feature = "lang-ru")]
            Self::Ru => "Русский",
            #[cfg(feature = "lang-ko")]
            Self::Ko => "한국어"
        }
    }
}

/// Define localized messages with compile-time validation.
///
/// Creates an enum with localized strings accessible via `.get(lang)` method.
/// All strings are `&'static str` with zero runtime allocation.
///
/// # Example
///
/// ```ignore
/// define_messages! {
///     pub enum UiMsg {
///         LabelWhy {
///             en: "Why:",
///             ru: "Почему:",
///             ko: "왜:",
///         }
///     }
/// }
///
/// let msg = UiMsg::LabelWhy.get(Lang::Ru);
/// assert_eq!(msg, "Почему:");
/// ```
#[macro_export]
macro_rules! define_messages {
    (
        $vis:vis enum $name:ident {
            $(
                $key:ident {
                    en: $en:literal
                    $(, ru: $ru:literal)?
                    $(, ko: $ko:literal)?
                    $(,)?
                }
            )*
        }
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u16)]
        $vis enum $name {
            $($key,)*
        }

        impl $name {
            /// Total number of messages.
            pub const COUNT: usize = {
                let mut count = 0usize;
                $(let _ = stringify!($key); count += 1;)*
                count
            };

            /// Get localized string for this message key.
            #[inline]
            pub const fn get(self, lang: $crate::i18n::Lang) -> &'static str {
                match self {
                    $(
                        Self::$key => {
                            match lang {
                                $crate::i18n::Lang::En => $en,
                                $(
                                    #[cfg(feature = "lang-ru")]
                                    $crate::i18n::Lang::Ru => $ru,
                                )?
                                $(
                                    #[cfg(feature = "lang-ko")]
                                    $crate::i18n::Lang::Ko => $ko,
                                )?
                                #[allow(unreachable_patterns)]
                                _ => $en,
                            }
                        }
                    )*
                }
            }

            /// Get all message keys as static slice.
            pub const fn all() -> &'static [Self] {
                &[$(Self::$key,)*]
            }
        }
    };
}

pub use define_messages;
