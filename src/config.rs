use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct MsiConfig {
    pub(crate) author: Option<String>,
}
