use flexstr::LocalStr;
use serde::Deserialize;

/// # [Product Information Properties](https://learn.microsoft.com/en-us/windows/win32/msi/property-reference)
///
/// All properties in this section are required for all MSI installations.
///
/// ## Properties
///
/// - [*`product_name`*](https://learn.microsoft.com/en-us/windows/win32/msi/productname)
///   The name of the application to be installed.
///
/// - [`product_version`](https://learn.microsoft.com/en-us/windows/win32/msi/productversion)
///   The version of the application to be installed. The format is
///   \[MAJOR].\[MINOR].\[BUILD]
///
/// - [`manufacturer`](https://learn.microsoft.com/en-us/windows/win32/msi/manufacturer)
///   The name of the manufacturer for the application that is being installed.
///
/// - [`product_language`](https://learn.microsoft.com/en-us/windows/win32/msi/productlanguage)
///   Specifies the language the installer should use for any strings in the
///   user interface that are not authored into the database. This property must
///   be a numeric language identifier.
///     - TODO: Figure out where to find the official definitions of these for
///       users. I believe English is 1033.
///
/// - [`product_code`](https://learn.microsoft.com/en-us/windows/win32/msi/productcode)
///   A unique identifier for the particular product release, represented as a
///   string GUID. This ID must vary for different versions and languages. Set
///   this to `*` to have the program generate the GUID automatically.
///
#[derive(Deserialize)]
#[serde(rename = "product_info")]
pub(crate) struct ProductInformationProperties {
    pub(crate) product_name: LocalStr,
    pub(crate) product_version: LocalStr,
    pub(crate) manufacturer: LocalStr,
    pub(crate) product_language: u16,
    pub(crate) product_code: LocalStr,
}
