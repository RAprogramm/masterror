use masterror::Masterror;

#[derive(Debug, Masterror)]
#[error("no attribute")]
struct Missing;

fn main() {}
