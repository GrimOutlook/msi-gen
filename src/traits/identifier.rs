use flexstr::LocalStr;
use uuid::Uuid;

pub(crate) trait Identifier {
    fn as_identifier() -> LocalStr;
}

impl Identifier for Uuid {
    fn as_identifier() -> LocalStr {
        // We make all identifiers start with an underscore so we can ignore the
        // case when the UUID starts with a number, which is invalid for
        // identifiers.
        ("_".to_string() + Uuid::new_v4().simple().to_string().as_str()).into()
    }
}
