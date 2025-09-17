use masterror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
struct TransparentUnit;
