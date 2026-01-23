// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! English locale.

use std::collections::HashMap;

pub fn messages() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    // Labels
    m.insert("label-translation", "Translation:");
    m.insert("label-why", "Why:");
    m.insert("label-fix", "Fix:");
    m.insert("label-link", "Learn more:");

    // Categories
    m.insert("category-ownership", "Ownership");
    m.insert("category-types", "Types");
    m.insert("category-lifetimes", "Lifetimes");
    m.insert("category-borrowing", "Borrowing");

    // E0382 - Use of moved value
    m.insert("e0382-title", "Use of moved value");
    m.insert(
        "e0382-explanation",
        "\
In Rust, each value has exactly one owner at a time. This is the foundation
of Rust's memory safety guarantees without garbage collection.

When you assign a value to another variable or pass it to a function,
ownership MOVES to the new location. The original variable becomes invalid
and cannot be used anymore.

This happens because Rust needs to know exactly when to free memory.
With one owner, there's no ambiguity about who is responsible for cleanup."
    );
    m.insert(
        "e0382-fix-clone-desc",
        "Clone the value (creates a deep copy)"
    );
    m.insert(
        "e0382-fix-borrow-desc",
        "Borrow with a reference (no copy, shared access)"
    );
    m.insert(
        "e0382-fix-copy-desc",
        "Implement Copy trait (for small, stack-only types)"
    );

    // E0502 - Cannot borrow as mutable (already borrowed as immutable)
    m.insert(
        "e0502-title",
        "Cannot borrow as mutable (already borrowed as immutable)"
    );
    m.insert(
        "e0502-explanation",
        "\
Rust enforces a strict borrowing rule: you can have EITHER one mutable
reference OR any number of immutable references, but never both at once.

This prevents data races at compile time. If you could mutate data while
someone else is reading it, the reader might see inconsistent state.

The immutable borrow is still \"active\" because it's used later in code.
Rust tracks lifetimes to ensure references don't outlive their data."
    );
    m.insert(
        "e0502-fix-scope-desc",
        "End the immutable borrow before mutating"
    );
    m.insert("e0502-fix-clone-desc", "Clone the data before mutation");

    // E0499 - Cannot borrow as mutable more than once
    m.insert("e0499-title", "Cannot borrow as mutable more than once");
    m.insert(
        "e0499-explanation",
        "\
Rust allows only ONE mutable reference to data at a time. This is stricter
than the immutable borrowing rule and prevents all aliased mutation.

Why? Two mutable references to the same data could lead to:
- Data races in concurrent code
- Iterator invalidation
- Dangling pointers after reallocation

This rule is checked at compile time, giving you fearless concurrency."
    );
    m.insert(
        "e0499-fix-scope-desc",
        "Use scopes to limit mutable borrow lifetime"
    );
    m.insert(
        "e0499-fix-refcell-desc",
        "Use RefCell for interior mutability (runtime checks)"
    );

    // E0308 - Mismatched types
    m.insert("e0308-title", "Mismatched types");
    m.insert(
        "e0308-explanation",
        "\
Rust is statically typed and does NOT perform implicit type conversions.
Every value has a specific type, and the compiler ensures type consistency.

This catches bugs at compile time that would be runtime errors in other
languages. The type system is your friend, not an obstacle."
    );
    m.insert(
        "e0308-fix-convert-desc",
        "Use parse() for string to number conversion"
    );
    m.insert("e0308-fix-as-desc", "Use 'as' for numeric type casting");

    // E0106 - Missing lifetime specifier
    m.insert("e0106-title", "Missing lifetime specifier");
    m.insert(
        "e0106-explanation",
        "\
References in Rust have lifetimes - they describe how long the reference
is valid. Usually the compiler infers lifetimes, but sometimes you must
be explicit.

Lifetime annotations don't change how long values live. They describe
relationships between references so the compiler can verify safety."
    );
    m.insert("e0106-fix-lifetime-desc", "Add explicit lifetime parameter");
    m.insert(
        "e0106-fix-owned-desc",
        "Use owned type instead of reference"
    );

    // E0597 - Value does not live long enough
    m.insert("e0597-title", "Value does not live long enough");
    m.insert(
        "e0597-explanation",
        "\
You're creating a reference to something that will be destroyed before
the reference is used. This would create a dangling pointer.

Rust prevents this at compile time. The referenced value must live at
least as long as the reference itself."
    );
    m.insert("e0597-fix-move-desc", "Move value to outer scope");
    m.insert(
        "e0597-fix-owned-desc",
        "Return owned value instead of reference"
    );

    m
}
