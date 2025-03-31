use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct MsiConfig {
    pub(crate) product_info: ProductInformationProperties,
}

/// # [Properties](https://learn.microsoft.com/en-us/windows/win32/msi/property-reference)
/// ## Required
/// ### [`product_name`](https://learn.microsoft.com/en-us/windows/win32/msi/productname)
/// The name of the application to be installed.
///
/// ### [`product_version`](https://learn.microsoft.com/en-us/windows/win32/msi/productversion)
/// The version of the application to be installed. The format is
/// \[MAJOR].\[MINOR].\[BUILD]
///
/// ### [`manufacturer`](https://learn.microsoft.com/en-us/windows/win32/msi/manufacturer)
/// The name of the manufacturer for the application that is being installed.
///
/// ### [`product_language`](https://learn.microsoft.com/en-us/windows/win32/msi/productlanguage)
/// Specifies the language the installer should use for any strings in the user
/// interface that are not authored into the database. This property must be a
/// numeric language identifier.
///
/// ### [`product_code`](https://learn.microsoft.com/en-us/windows/win32/msi/productcode)
/// A unique identifier for the particular product release, represented as a
/// string GUID. This ID must vary for different versions and languages.
///
/// ## Optional
/// ### `author`
/// The name of the author publishing the application.
///
/// I cannot find any reference to this property in the microsoft documentation
///
/// TODO: Track down documentation for this field
///
#[derive(Deserialize)]
#[serde(rename = "product_info")]
pub(crate) struct ProductInformationProperties {
    // Required Properties
    pub(crate) product_name: String,
    pub(crate) product_version: String,
    pub(crate) manufacturer: String,
    pub(crate) product_language: String,
    pub(crate) product_code: String,
    // Optional Properties
    pub(crate) author: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONFIG: &str = r#"
[product_info]
product_name = "Test Application"
author = "Test Name"
product_version = "22.1.15"
manufacturer = "Myself"
"#;

    #[test]
    fn config_deserializes() {
        let c: MsiConfig = toml::from_str(TEST_CONFIG).unwrap();
        // Required Properties
        assert_eq!(c.product_info.product_name, "Test Application");
        assert_eq!(c.product_info.product_version, "22.1.15");
        assert_eq!(c.product_info.manufacturer, "Myself");

        // Optional properties
        assert!(c.product_info.author.is_some());
        assert_eq!(c.product_info.author.unwrap(), "Test Name");
    }
}
