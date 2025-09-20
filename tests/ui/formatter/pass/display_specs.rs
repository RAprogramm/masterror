use masterror::Error;

#[derive(Debug, Error)]
#[error("{value:>8}", value = .value)]
struct Alignment {
    value: &'static str,
}

#[derive(Debug, Error)]
#[error("{value:.3}", value = .value)]
struct Precision {
    value: f64,
}

#[derive(Debug, Error)]
#[error("{value:*<6}", value = .value)]
struct Fill {
    value: &'static str,
}

fn main() {}
