// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Test file with common Rust errors for masterror-cli testing.
//!
//! This file contains INTENTIONAL compile errors to test masterror-cli.
//! It should NOT be compiled directly - only used with `masterror check`.
//!
//! Run: `cd examples/cli-test-errors && cargo masterror check`
//!
//! To see all masterror features:
//! ```sh
//! # List all known errors
//! masterror list
//!
//! # List by category
//! masterror list -c ownership
//!
//! # Explain specific error
//! masterror explain E0382
//!
//! # List best practices
//! masterror practice
//!
//! # Explain best practice
//! masterror explain RA001
//!
//! # Use Russian language
//! masterror -l ru list
//! masterror -l ru explain E0382
//!
//! # Use Korean language
//! masterror -l ko list
//! ```

fn main() {
    // ==================== OWNERSHIP ERRORS ====================

    // E0382 - Borrow of moved value
    let s = String::from("hello");
    let s2 = s;
    println!("{}", s); // error: value moved

    // E0381 - Borrow of possibly-uninitialized variable
    let x: i32;
    println!("{}", x); // error: possibly uninitialized

    // E0384 - Cannot assign twice to immutable variable
    let y = 5;
    y = 10; // error: immutable variable

    // E0505 - Cannot move out because it is borrowed
    let mut data = String::from("data");
    let r = &data;
    let moved = data; // error: borrowed
    println!("{}", r);

    // E0507 - Cannot move out of borrowed content
    let borrowed = &String::from("borrowed");
    let owned = *borrowed; // error: cannot move out of borrowed

    // ==================== BORROWING ERRORS ====================

    // E0502 - Cannot borrow as mutable (already borrowed as immutable)
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4); // error: mutable borrow while immutable exists
    println!("{}", first);

    // E0499 - Cannot borrow as mutable more than once
    let mut num = 5;
    let r1 = &mut num;
    let r2 = &mut num; // error: second mutable borrow
    println!("{} {}", r1, r2);

    // E0596 - Cannot borrow as mutable (not declared as mutable)
    let vec = vec![1, 2, 3];
    vec.push(4); // error: not mutable

    // E0503 - Cannot use value because it was mutably borrowed
    let mut borrowed_mut = 10;
    let r = &mut borrowed_mut;
    let copy = borrowed_mut; // error: mutably borrowed
    *r = 20;

    // ==================== LIFETIME ERRORS ====================

    // E0106 - Missing lifetime specifier
    struct MissingLifetime {
        data: &str, // error: missing lifetime
    }

    // E0597 - Value does not live long enough
    let reference;
    {
        let short_lived = String::from("short");
        reference = &short_lived; // error: does not live long enough
    }
    println!("{}", reference);

    // E0515 - Cannot return reference to temporary value
    fn returns_ref() -> &str {
        &String::from("temp") // error: returns reference to temporary
    }

    // ==================== TYPE ERRORS ====================

    // E0308 - Mismatched types
    let number: i32 = "not a number"; // error: expected i32, found &str

    // E0277 - Trait bound not satisfied
    fn requires_display<T: std::fmt::Display>(t: T) {}
    struct NoDisplay;
    requires_display(NoDisplay); // error: Display not implemented

    // E0599 - Method not found
    let val = 42;
    val.nonexistent_method(); // error: method not found

    // ==================== TRAIT ERRORS ====================

    // E0038 - Cannot be made into an object
    trait NotObjectSafe {
        fn generic<T>(&self);
    }
    fn takes_dyn(obj: &dyn NotObjectSafe) {} // error: not object safe

    // E0282 - Type annotations needed
    let needs_type = Default::default(); // error: cannot infer type

    // ==================== RESOLUTION ERRORS ====================

    // E0412 - Cannot find type in this scope
    let value: NonExistentType = todo!(); // error: type not found

    // E0425 - Cannot find value in this scope
    let result = undefined_variable + 1; // error: value not found

    // E0433 - Failed to resolve: use of undeclared crate or module
    use nonexistent::module; // error: module not found
}
