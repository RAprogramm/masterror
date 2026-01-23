// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Knowledge base for Rust compiler errors.
#![allow(dead_code)]

/// Link with title for documentation.
pub struct DocLink {
    pub title: &'static str,
    pub url:   &'static str
}

/// Fix suggestion with code example.
pub struct FixSuggestion {
    pub description_key: &'static str,
    pub code:            &'static str
}

/// Entry in the error knowledge base.
pub struct ErrorEntry {
    pub code:            &'static str,
    pub title_key:       &'static str,
    pub category:        &'static str,
    pub explanation_key: &'static str,
    pub fixes:           &'static [FixSuggestion],
    pub links:           &'static [DocLink]
}

/// Get all known error entries.
pub fn entries() -> &'static [ErrorEntry] {
    &[
        ErrorEntry {
            code:            "E0382",
            title_key:       "e0382-title",
            category:        "ownership",
            explanation_key: "e0382-explanation",
            fixes:           &[
                FixSuggestion {
                    description_key: "e0382-fix-clone-desc",
                    code:            "let s2 = s.clone();"
                },
                FixSuggestion {
                    description_key: "e0382-fix-borrow-desc",
                    code:            "let s2 = &s;"
                },
                FixSuggestion {
                    description_key: "e0382-fix-copy-desc",
                    code:            "#[derive(Copy, Clone)]"
                }
            ],
            links:           &[
                DocLink {
                    title: "Rust Book: Ownership",
                    url:   "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html"
                },
                DocLink {
                    title: "Rust Reference",
                    url:   "https://doc.rust-lang.org/error_codes/E0382.html"
                },
                DocLink {
                    title: "Rust By Example",
                    url:   "https://doc.rust-lang.org/rust-by-example/scope/move.html"
                }
            ]
        },
        ErrorEntry {
            code:            "E0502",
            title_key:       "e0502-title",
            category:        "borrowing",
            explanation_key: "e0502-explanation",
            fixes:           &[
                FixSuggestion {
                    description_key: "e0502-fix-scope-desc",
                    code:            "{ let r = &x; println!(\"{}\", r); } // r dropped\nx.push(1);"
                },
                FixSuggestion {
                    description_key: "e0502-fix-clone-desc",
                    code:            "let first = v[0].clone(); // clone before mutation"
                }
            ],
            links:           &[
                DocLink {
                    title: "Rust Book: References and Borrowing",
                    url:   "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html"
                },
                DocLink {
                    title: "Rust Reference",
                    url:   "https://doc.rust-lang.org/error_codes/E0502.html"
                }
            ]
        },
        ErrorEntry {
            code:            "E0499",
            title_key:       "e0499-title",
            category:        "borrowing",
            explanation_key: "e0499-explanation",
            fixes:           &[
                FixSuggestion {
                    description_key: "e0499-fix-scope-desc",
                    code:            "{ let r1 = &mut x; *r1 += 1; } // r1 dropped\nlet r2 = &mut x;"
                },
                FixSuggestion {
                    description_key: "e0499-fix-refcell-desc",
                    code:            "use std::cell::RefCell;\nlet x = RefCell::new(value);"
                }
            ],
            links:           &[
                DocLink {
                    title: "Rust Book: Mutable References",
                    url:   "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references"
                },
                DocLink {
                    title: "Rust Reference",
                    url:   "https://doc.rust-lang.org/error_codes/E0499.html"
                }
            ]
        },
        ErrorEntry {
            code:            "E0308",
            title_key:       "e0308-title",
            category:        "types",
            explanation_key: "e0308-explanation",
            fixes:           &[
                FixSuggestion {
                    description_key: "e0308-fix-convert-desc",
                    code:            "let n: i32 = s.parse().unwrap();"
                },
                FixSuggestion {
                    description_key: "e0308-fix-as-desc",
                    code:            "let n = x as i32;"
                }
            ],
            links:           &[
                DocLink {
                    title: "Rust Book: Data Types",
                    url:   "https://doc.rust-lang.org/book/ch03-02-data-types.html"
                },
                DocLink {
                    title: "Rust Reference",
                    url:   "https://doc.rust-lang.org/error_codes/E0308.html"
                }
            ]
        },
        ErrorEntry {
            code:            "E0106",
            title_key:       "e0106-title",
            category:        "lifetimes",
            explanation_key: "e0106-explanation",
            fixes:           &[
                FixSuggestion {
                    description_key: "e0106-fix-lifetime-desc",
                    code:            "struct Foo<'a> { x: &'a str }"
                },
                FixSuggestion {
                    description_key: "e0106-fix-owned-desc",
                    code:            "struct Foo { x: String }"
                }
            ],
            links:           &[
                DocLink {
                    title: "Rust Book: Lifetimes",
                    url:   "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
                },
                DocLink {
                    title: "Rust Reference",
                    url:   "https://doc.rust-lang.org/error_codes/E0106.html"
                }
            ]
        },
        ErrorEntry {
            code:            "E0597",
            title_key:       "e0597-title",
            category:        "lifetimes",
            explanation_key: "e0597-explanation",
            fixes:           &[
                FixSuggestion {
                    description_key: "e0597-fix-move-desc",
                    code:            "let s = String::from(\"hello\"); // move to outer scope"
                },
                FixSuggestion {
                    description_key: "e0597-fix-owned-desc",
                    code:            "return s.to_string(); // return owned value"
                }
            ],
            links:           &[
                DocLink {
                    title: "Rust Book: Lifetimes",
                    url:   "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
                },
                DocLink {
                    title: "Rust Reference",
                    url:   "https://doc.rust-lang.org/error_codes/E0597.html"
                }
            ]
        }
    ]
}

/// Find error entry by code.
pub fn find(code: &str) -> Option<&'static ErrorEntry> {
    let normalized = if code.starts_with('E') || code.starts_with('e') {
        code.to_uppercase()
    } else {
        format!("E{code}")
    };
    entries().iter().find(|e| e.code == normalized)
}
