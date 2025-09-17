use masterror::Error;

#[derive(Debug, Error)]
enum TransparentEnumUnit {
    #[error(transparent)]
    Variant
}
