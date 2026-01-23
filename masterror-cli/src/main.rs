// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! masterror CLI - Rust compiler error explainer.

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::{Command, Stdio}
};

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use serde::Deserialize;

// ─────────────────────────────────────────────────────────────────────────────
// CLI Definition
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Parser, Debug)]
#[command(name = "masterror")]
#[command(author, version, about = "Rust compiler error explainer")]
#[command(propagate_version = true)]
struct Cli {
    /// Language for explanations (en, ru)
    #[arg(short, long, env = "MASTERROR_LANG", default_value = "en")]
    lang: String,

    /// Disable colored output
    #[arg(long, env = "NO_COLOR")]
    no_color: bool,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run cargo check and explain errors
    Check {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>
    },
    /// Explain a specific error code
    Explain { code: String },
    /// List all known error codes
    List {
        #[arg(short, long)]
        category: Option<String>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Localization
// ─────────────────────────────────────────────────────────────────────────────

struct Locale {
    messages: HashMap<&'static str, &'static str>
}

impl Locale {
    fn new(lang: &str) -> Self {
        let messages = match lang {
            "ru" => Self::russian(),
            _ => Self::english()
        };
        Self {
            messages
        }
    }

    fn get(&self, key: &'static str) -> &'static str {
        self.messages.get(key).copied().unwrap_or(key)
    }

    fn english() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        m.insert("label-why", "📖 Why?");
        m.insert("label-fix", "💡 How to fix?");
        m.insert("label-link", "🔗 Learn more:");
        m.insert("label-example", "📝 Example:");
        m.insert("category-ownership", "Ownership");
        m.insert("category-types", "Types");
        m.insert("category-lifetimes", "Lifetimes");
        m.insert("category-borrowing", "Borrowing");

        // E0382
        m.insert("e0382-title", "Use of moved value");
        m.insert("e0382-explanation", "In Rust, each value has exactly one owner. When you assign\n   a value to another variable, ownership MOVES. The original\n   variable becomes invalid and cannot be used anymore.");
        m.insert("e0382-fix-clone", "Clone the value: let s2 = s.clone();");
        m.insert("e0382-fix-borrow", "Borrow with reference: let s2 = &s;");

        // E0308
        m.insert("e0308-title", "Mismatched types");
        m.insert("e0308-explanation", "Rust is statically typed. You're using a value of one type\n   where a different type is expected. Rust doesn't do automatic\n   type conversion.");
        m.insert("e0308-fix-type", "Change the type annotation to match");
        m.insert(
            "e0308-fix-convert",
            "Convert using .parse(), .into(), or as"
        );

        // E0502
        m.insert(
            "e0502-title",
            "Cannot borrow as mutable (already borrowed as immutable)"
        );
        m.insert("e0502-explanation", "Rust's rule: you can have ONE mutable reference OR any number\n   of immutable references, but not both at the same time.");
        m.insert(
            "e0502-fix",
            "End the immutable borrow before creating a mutable one"
        );

        // E0499
        m.insert("e0499-title", "Cannot borrow as mutable more than once");
        m.insert(
            "e0499-explanation",
            "You can only have one mutable reference at a time.\n   This prevents data races."
        );
        m.insert(
            "e0499-fix",
            "Use scopes to ensure only one mutable borrow exists"
        );

        // E0106
        m.insert("e0106-title", "Missing lifetime specifier");
        m.insert("e0106-explanation", "References in structs need lifetime annotations.\n   They tell the compiler how long the reference is valid.");
        m.insert(
            "e0106-fix-lifetime",
            "Add lifetime: struct Foo<'a> { x: &'a str }"
        );
        m.insert("e0106-fix-owned", "Use owned type: String instead of &str");

        // E0597
        m.insert("e0597-title", "Value does not live long enough");
        m.insert("e0597-explanation", "You're creating a reference to something that will be\n   destroyed before the reference is used.");
        m.insert(
            "e0597-fix",
            "Move the value to a scope where it lives long enough"
        );

        m
    }

    fn russian() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        m.insert("label-why", "📖 Почему?");
        m.insert("label-fix", "💡 Как исправить?");
        m.insert("label-link", "🔗 Подробнее:");
        m.insert("label-example", "📝 Пример:");
        m.insert("category-ownership", "Владение (Ownership)");
        m.insert("category-types", "Типы");
        m.insert("category-lifetimes", "Времена жизни (Lifetimes)");
        m.insert("category-borrowing", "Заимствование (Borrowing)");

        // E0382
        m.insert("e0382-title", "Использование перемещённого значения");
        m.insert("e0382-explanation", "В Rust у каждого значения один владелец. Когда ты присваиваешь\n   значение другой переменной, владение ПЕРЕМЕЩАЕТСЯ. Старая\n   переменная становится недействительной.");
        m.insert("e0382-fix-clone", "Клонируй: let s2 = s.clone();");
        m.insert("e0382-fix-borrow", "Заимствуй: let s2 = &s;");

        // E0308
        m.insert("e0308-title", "Несовпадение типов");
        m.insert("e0308-explanation", "Rust статически типизирован. Ты используешь значение одного\n   типа там, где ожидается другой. Rust не делает автоматическое\n   преобразование типов.");
        m.insert("e0308-fix-type", "Измени аннотацию типа");
        m.insert(
            "e0308-fix-convert",
            "Преобразуй через .parse(), .into() или as"
        );

        // E0502
        m.insert(
            "e0502-title",
            "Нельзя заимствовать как изменяемое (уже заимствовано как неизменяемое)"
        );
        m.insert("e0502-explanation", "Правило Rust: можно иметь ОДНУ изменяемую ссылку ИЛИ любое\n   количество неизменяемых, но не оба одновременно.");
        m.insert(
            "e0502-fix",
            "Заверши неизменяемое заимствование перед созданием изменяемого"
        );

        // E0499
        m.insert(
            "e0499-title",
            "Нельзя заимствовать как изменяемое больше одного раза"
        );
        m.insert("e0499-explanation", "Можно иметь только одну изменяемую ссылку одновременно.\n   Это предотвращает гонки данных.");
        m.insert(
            "e0499-fix",
            "Используй области видимости для одного изменяемого заимствования"
        );

        // E0106
        m.insert("e0106-title", "Отсутствует спецификатор времени жизни");
        m.insert("e0106-explanation", "Ссылки в структурах требуют аннотации времени жизни.\n   Они говорят компилятору, как долго ссылка валидна.");
        m.insert(
            "e0106-fix-lifetime",
            "Добавь время жизни: struct Foo<'a> { x: &'a str }"
        );
        m.insert(
            "e0106-fix-owned",
            "Используй владеющий тип: String вместо &str"
        );

        // E0597
        m.insert("e0597-title", "Значение живёт недостаточно долго");
        m.insert(
            "e0597-explanation",
            "Ты создаёшь ссылку на что-то, что будет уничтожено\n   до использования ссылки."
        );
        m.insert(
            "e0597-fix",
            "Перемести значение туда, где оно живёт достаточно долго"
        );

        m
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Knowledge Base
// ─────────────────────────────────────────────────────────────────────────────

struct ErrorEntry {
    code:            &'static str,
    title_key:       &'static str,
    category:        &'static str,
    explanation_key: &'static str,
    fixes:           Vec<&'static str>,
    doc_url:         &'static str
}

fn get_knowledge_base() -> Vec<ErrorEntry> {
    vec![
        ErrorEntry {
            code:            "E0382",
            title_key:       "e0382-title",
            category:        "ownership",
            explanation_key: "e0382-explanation",
            fixes:           vec!["e0382-fix-clone", "e0382-fix-borrow"],
            doc_url:         "https://doc.rust-lang.org/error_codes/E0382.html"
        },
        ErrorEntry {
            code:            "E0308",
            title_key:       "e0308-title",
            category:        "types",
            explanation_key: "e0308-explanation",
            fixes:           vec!["e0308-fix-type", "e0308-fix-convert"],
            doc_url:         "https://doc.rust-lang.org/error_codes/E0308.html"
        },
        ErrorEntry {
            code:            "E0502",
            title_key:       "e0502-title",
            category:        "borrowing",
            explanation_key: "e0502-explanation",
            fixes:           vec!["e0502-fix"],
            doc_url:         "https://doc.rust-lang.org/error_codes/E0502.html"
        },
        ErrorEntry {
            code:            "E0499",
            title_key:       "e0499-title",
            category:        "borrowing",
            explanation_key: "e0499-explanation",
            fixes:           vec!["e0499-fix"],
            doc_url:         "https://doc.rust-lang.org/error_codes/E0499.html"
        },
        ErrorEntry {
            code:            "E0106",
            title_key:       "e0106-title",
            category:        "lifetimes",
            explanation_key: "e0106-explanation",
            fixes:           vec!["e0106-fix-lifetime", "e0106-fix-owned"],
            doc_url:         "https://doc.rust-lang.org/error_codes/E0106.html"
        },
        ErrorEntry {
            code:            "E0597",
            title_key:       "e0597-title",
            category:        "lifetimes",
            explanation_key: "e0597-explanation",
            fixes:           vec!["e0597-fix"],
            doc_url:         "https://doc.rust-lang.org/error_codes/E0597.html"
        },
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// Cargo JSON Parser
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct CargoMessage {
    reason:  String,
    message: Option<DiagnosticMessage>
}

#[derive(Deserialize)]
struct DiagnosticMessage {
    level:   String,
    message: String,
    code:    Option<DiagnosticCode>,
    spans:   Vec<DiagnosticSpan>
}

#[derive(Deserialize)]
struct DiagnosticCode {
    code: String
}

#[derive(Deserialize)]
struct DiagnosticSpan {
    file_name:    String,
    line_start:   usize,
    column_start: usize,
    is_primary:   bool
}

// ─────────────────────────────────────────────────────────────────────────────
// Commands
// ─────────────────────────────────────────────────────────────────────────────

fn cmd_check(
    locale: &Locale,
    args: &[String],
    colored: bool
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("cargo")
        .arg("check")
        .arg("--message-format=json")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let stdout = cmd.stdout.take().expect("stdout captured");
    let reader = BufReader::new(stdout);
    let kb = get_knowledge_base();

    let mut error_count = 0;

    for line in reader.lines() {
        let line = line?;
        if let Ok(msg) = serde_json::from_str::<CargoMessage>(&line)
            && msg.reason == "compiler-message"
            && let Some(diag) = msg.message
            && diag.level == "error"
        {
            error_count += 1;
            print_error(locale, &kb, &diag, colored);
        }
    }

    let status = cmd.wait()?;

    if error_count > 0 {
        println!();
        println!(
            "Found {} error(s). Run `masterror explain <code>` for details.",
            error_count
        );
    }

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

fn print_error(locale: &Locale, kb: &[ErrorEntry], diag: &DiagnosticMessage, colored: bool) {
    println!();

    let code_str = diag
        .code
        .as_ref()
        .map(|c| c.code.as_str())
        .unwrap_or("unknown");

    // Header
    if colored {
        println!("{} {}", "❌".red(), code_str.red().bold());
    } else {
        println!("❌ {}", code_str);
    }

    println!("   {}", diag.message);

    // Location
    for span in diag.spans.iter().filter(|s| s.is_primary) {
        if colored {
            println!(
                "   {} {}:{}:{}",
                "-->".blue(),
                span.file_name,
                span.line_start,
                span.column_start
            );
        } else {
            println!(
                "   --> {}:{}:{}",
                span.file_name, span.line_start, span.column_start
            );
        }
    }

    // Knowledge base lookup
    if let Some(code) = &diag.code
        && let Some(entry) = kb.iter().find(|e| e.code == code.code)
    {
        println!();
        let why = locale.get("label-why");
        if colored {
            println!("{}", why.yellow().bold());
        } else {
            println!("{}", why);
        }
        println!("   {}", locale.get(entry.explanation_key));

        if !entry.fixes.is_empty() {
            println!();
            let fix = locale.get("label-fix");
            if colored {
                println!("{}", fix.green().bold());
            } else {
                println!("{}", fix);
            }
            for fix_key in &entry.fixes {
                println!("   • {}", locale.get(fix_key));
            }
        }

        println!();
        let link = locale.get("label-link");
        if colored {
            println!("{} {}", link.cyan(), entry.doc_url.underline().cyan());
        } else {
            println!("{} {}", link, entry.doc_url);
        }
    }
}

fn cmd_explain(
    locale: &Locale,
    code: &str,
    colored: bool
) -> Result<(), Box<dyn std::error::Error>> {
    let kb = get_knowledge_base();

    // Normalize code
    let normalized = if code.starts_with('E') || code.starts_with('e') {
        code.to_uppercase()
    } else {
        format!("E{}", code)
    };

    let Some(entry) = kb.iter().find(|e| e.code == normalized) else {
        eprintln!("Unknown error code: {}", normalized);
        eprintln!("Run `masterror list` to see available codes.");
        std::process::exit(1);
    };

    println!();

    // Title
    let title = locale.get(entry.title_key);
    if colored {
        println!(
            "{} {} - {}",
            "📖".yellow(),
            normalized.yellow().bold(),
            title.bold()
        );
    } else {
        println!("📖 {} - {}", normalized, title);
    }

    // Category
    println!();
    let cat_key = format!("category-{}", entry.category);
    let category = get_category_name(locale, &cat_key);
    if colored {
        println!("Category: {}", category.dimmed());
    } else {
        println!("Category: {}", category);
    }

    // Explanation
    println!();
    let why = locale.get("label-why");
    if colored {
        println!("{}", why.yellow().bold());
    } else {
        println!("{}", why);
    }
    println!("   {}", locale.get(entry.explanation_key));

    // Fixes
    if !entry.fixes.is_empty() {
        println!();
        let fix = locale.get("label-fix");
        if colored {
            println!("{}", fix.green().bold());
        } else {
            println!("{}", fix);
        }
        for fix_key in &entry.fixes {
            println!("   • {}", locale.get(fix_key));
        }
    }

    // Link
    println!();
    let link = locale.get("label-link");
    if colored {
        println!("{} {}", link.cyan(), entry.doc_url.underline().cyan());
    } else {
        println!("{} {}", link, entry.doc_url);
    }

    println!();
    Ok(())
}

fn get_category_name(locale: &Locale, key: &str) -> &'static str {
    match key {
        "category-ownership" => locale.get("category-ownership"),
        "category-types" => locale.get("category-types"),
        "category-lifetimes" => locale.get("category-lifetimes"),
        "category-borrowing" => locale.get("category-borrowing"),
        _ => "Unknown"
    }
}

fn cmd_list(
    locale: &Locale,
    category: Option<&str>,
    colored: bool
) -> Result<(), Box<dyn std::error::Error>> {
    let kb = get_knowledge_base();

    println!();
    if colored {
        println!("{}", "Known Rust Compiler Errors".bold());
    } else {
        println!("Known Rust Compiler Errors");
    }
    println!();

    let filtered: Vec<_> = if let Some(cat) = category {
        kb.iter()
            .filter(|e| e.category.eq_ignore_ascii_case(cat))
            .collect()
    } else {
        kb.iter().collect()
    };

    if filtered.is_empty() {
        println!("   No errors found.");
        return Ok(());
    }

    let mut current_cat = "";
    for entry in &filtered {
        if entry.category != current_cat {
            current_cat = entry.category;
            println!();
            let cat_name = get_category_name(locale, &format!("category-{}", current_cat));
            if colored {
                println!("  {}", cat_name.yellow().bold());
            } else {
                println!("  {}", cat_name);
            }
            println!();
        }

        let title = locale.get(entry.title_key);
        if colored {
            println!("    {} - {}", entry.code.cyan(), title);
        } else {
            println!("    {} - {}", entry.code, title);
        }
    }

    println!();
    println!("Total: {} errors", filtered.len());
    println!();

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Main
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();
    let locale = Locale::new(&cli.lang);
    let colored = !cli.no_color;

    let result = match cli.command {
        Commands::Check {
            ref args
        } => cmd_check(&locale, args, colored),
        Commands::Explain {
            ref code
        } => cmd_explain(&locale, code, colored),
        Commands::List {
            ref category
        } => cmd_list(&locale, category.as_deref(), colored)
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
