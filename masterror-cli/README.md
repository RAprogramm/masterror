<!--
SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# masterror-cli

CLI tool for explaining Rust compiler errors in human-friendly language.

## Installation

```bash
cargo install masterror-cli
```

## Usage

### Check and explain errors

```bash
# Run cargo check with explanations
masterror check

# Pass additional arguments to cargo
masterror check --release
masterror check -p my-crate
```

### Explain specific error

```bash
# Explain an error code
masterror explain E0382
masterror explain 0382  # E prefix is optional

# In Russian
masterror explain E0382 --lang ru
```

### List known errors

```bash
# List all known errors
masterror list

# Filter by category
masterror list --category ownership
masterror list --category lifetimes
```

## Language Support

Set language via flag or environment variable:

```bash
# Flag
masterror check --lang ru

# Environment variable
export MASTERROR_LANG=ru
masterror check
```

Supported languages:
- `en` - English (default)
- `ru` - Russian

## Example Output

```
❌ E0382 - Use of moved value

   let s = String::from("hello");
   let s2 = s;      // s moved to s2
   println!("{}", s); // error: s is no longer valid

   --> src/main.rs:5:10

📖 Why?
   In Rust, each value has exactly one owner. When you assign a value
   to another variable (like `let s2 = s`), ownership MOVES to the new
   variable. The original variable becomes invalid.

💡 How to fix?
   • Clone the value if you need two independent copies
     let s2 = s.clone();
   • Borrow with a reference if you just need to read
     let s2 = &s;

🔗 Learn more: https://doc.rust-lang.org/error_codes/E0382.html
```

## Contributing

Adding new error explanations:

1. Add `ErrorEntry` in `get_knowledge_base()` function in `src/main.rs`
2. Add translations in `Locale::english()` and `Locale::russian()` methods
3. Submit PR

## License

MIT
