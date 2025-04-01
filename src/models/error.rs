#[derive(Debug)]
pub(crate) struct MsiError {
    message: String,
    nested_error: Option<Box<dyn std::error::Error + 'static>>,
}

impl MsiError {
    pub fn short(message: impl ToString) -> MsiError {
        MsiError {
            message: message.to_string(),
            nested_error: None,
        }
    }

    pub fn nested(
        message: impl ToString,
        nested_error: impl Into<Box<dyn std::error::Error + 'static>>,
    ) -> MsiError {
        MsiError {
            message: message.to_string(),
            nested_error: Some(nested_error.into()),
        }
    }
}

impl std::fmt::Display for MsiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nested_error = match &self.nested_error {
            Some(n) => format!("\nNested Error: {}", n.to_string()),
            None => "".to_owned(),
        };
        let msg = format!("Error: {}{}", self.message, nested_error);
        write!(f, "{}", msg)
    }
}
