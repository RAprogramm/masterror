use masterror::Error;

#[derive(Debug, Error)]
#[error("{left:?} - {right:?}")]
struct BadStruct {
    #[from]
    left: DummyError,
    #[backtrace]
    trace: Option<std::backtrace::Backtrace>,
    right: DummyError,
}

#[derive(Debug, Error)]
#[error("dummy")] 
struct DummyError;

fn main() {}
