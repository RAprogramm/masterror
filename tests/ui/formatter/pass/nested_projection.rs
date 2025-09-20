use masterror::Error;

#[derive(Debug)]
struct Limits {
    lo: i32,
    hi: i32,
}

#[derive(Debug, Error)]
#[error(
    "range {lo}-{hi} suggestion {suggestion}",
    lo = .limits.lo,
    hi = .limits.hi,
    suggestion = .suggestion.as_ref().map_or_else(|| "<none>", |s| s.as_str())
)]
struct StructProjection {
    limits: Limits,
    suggestion: Option<String>,
}

#[derive(Debug)]
struct Payload {
    data: &'static str,
}

#[derive(Debug, Error)]
enum EnumProjection {
    #[error("tuple data {data}", data = .0.data)]
    Tuple(Payload),
    #[error(
        "named suggestion {value}",
        value = .suggestion.as_ref().map_or_else(|| "<none>", |s| s.as_str())
    )]
    Named { suggestion: Option<String> },
}

fn main() {
    let _ = StructProjection {
        limits: Limits { lo: 0, hi: 3 },
        suggestion: Some(String::from("hint")),
    };
    let _ = EnumProjection::Tuple(Payload { data: "payload" });
}
