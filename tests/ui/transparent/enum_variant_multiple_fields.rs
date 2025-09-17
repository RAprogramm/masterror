use masterror::Error;

#[derive(Debug, Error)]
enum TransparentEnumFail {
    #[error(transparent)]
    Variant(String, String)
}
