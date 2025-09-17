use masterror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
struct TransparentMany {
    first: String,
    second: String
}
