use flexstr::LocalStr;

type Inner = Box<dyn std::error::Error>;

#[derive(Debug)]
pub(crate) struct MsiError {
    message: LocalStr,
    inner: Option<Inner>,
}

impl MsiError {
    pub fn short(message: impl Into<LocalStr>) -> MsiError {
        MsiError {
            message: message.into(),
            inner: None,
        }
    }

    pub fn nested(message: impl Into<LocalStr>, inner: impl Into<Inner>) -> MsiError {
        MsiError {
            message: message.into(),
            inner: Some(inner.into()),
        }
    }

    pub fn inner(&self) -> &Option<Inner> {
        &self.inner
    }
}

impl std::fmt::Display for MsiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nested_error = match &self.inner {
            Some(n) => format!("\nCause: {}", n),
            None => "".to_owned(),
        };
        let msg = self.message.clone() + &nested_error;
        write!(f, "{}", msg)
    }
}
