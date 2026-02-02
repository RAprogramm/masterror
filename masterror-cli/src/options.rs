// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Display options for masterror output.

/// What sections to show in masterror block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayOptions {
    /// Enable colored output.
    pub colored:          bool,
    /// Show translated error message.
    pub show_translation: bool,
    /// Show "why this happens" explanation.
    pub show_why:         bool,
    /// Show fix suggestions.
    pub show_fix:         bool,
    /// Show documentation links.
    pub show_links:       bool,
    /// Show original compiler output.
    pub show_original:    bool
}

impl DisplayOptions {
    /// Default options as const value.
    pub const DEFAULT: Self = Self {
        colored:          true,
        show_translation: true,
        show_why:         true,
        show_fix:         true,
        show_links:       true,
        show_original:    false
    };

    /// Create new builder for constructing DisplayOptions.
    #[allow(dead_code)]
    pub const fn builder() -> DisplayOptionsBuilder {
        DisplayOptionsBuilder::new()
    }
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Builder for constructing DisplayOptions with const support.
///
/// # Example
///
/// ```ignore
/// use masterror_cli::options::DisplayOptions;
///
/// const OPTS: DisplayOptions = DisplayOptions::builder()
///     .colored(false)
///     .show_original(true)
///     .build();
/// ```
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct DisplayOptionsBuilder {
    colored:          bool,
    show_translation: bool,
    show_why:         bool,
    show_fix:         bool,
    show_links:       bool,
    show_original:    bool
}

#[allow(dead_code)]
impl DisplayOptionsBuilder {
    /// Create new builder with default values.
    pub const fn new() -> Self {
        Self {
            colored:          true,
            show_translation: true,
            show_why:         true,
            show_fix:         true,
            show_links:       true,
            show_original:    false
        }
    }

    /// Set colored output.
    pub const fn colored(mut self, value: bool) -> Self {
        self.colored = value;
        self
    }

    /// Set show translation.
    pub const fn show_translation(mut self, value: bool) -> Self {
        self.show_translation = value;
        self
    }

    /// Set show why explanation.
    pub const fn show_why(mut self, value: bool) -> Self {
        self.show_why = value;
        self
    }

    /// Set show fix suggestions.
    pub const fn show_fix(mut self, value: bool) -> Self {
        self.show_fix = value;
        self
    }

    /// Set show documentation links.
    pub const fn show_links(mut self, value: bool) -> Self {
        self.show_links = value;
        self
    }

    /// Set show original compiler output.
    pub const fn show_original(mut self, value: bool) -> Self {
        self.show_original = value;
        self
    }

    /// Build the DisplayOptions.
    pub const fn build(self) -> DisplayOptions {
        DisplayOptions {
            colored:          self.colored,
            show_translation: self.show_translation,
            show_why:         self.show_why,
            show_fix:         self.show_fix,
            show_links:       self.show_links,
            show_original:    self.show_original
        }
    }
}

impl Default for DisplayOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
