use serde::Deserialize;

use super::{product_information::ProductInformationProperties, summary_information::SummaryInformationProperties};

/// 
#[derive(Deserialize)]
pub(crate) struct MsiConfig {
    pub(crate) product_info: ProductInformationProperties,
    pub(crate) summary_info: SummaryInformationProperties,
}
