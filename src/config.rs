// TODO: Remove this when the library is done
#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct MsiConfig {
    pub(crate) product_info: ProductInformationProperties,
    pub(crate) summary_info: SummaryInformationProperties,
    pub(crate) default_files: Option<DefaultFiles>,
    pub(crate) explicit_files: Option<DefaultFiles>,
}

/// # [Product Information Properties](https://learn.microsoft.com/en-us/windows/win32/msi/property-reference)
///
/// All properties in this section are required for all MSI installations.
///
/// ## Properties
///
///  - [*`product_name`*](https://learn.microsoft.com/en-us/windows/win32/msi/productname)
///     The name of the application to be installed.
///
/// - [`product_version`](https://learn.microsoft.com/en-us/windows/win32/msi/productversion)
///     The version of the application to be installed. The format is
///     \[MAJOR].\[MINOR].\[BUILD]
///
/// - [`manufacturer`](https://learn.microsoft.com/en-us/windows/win32/msi/manufacturer)
///     The name of the manufacturer for the application that is being
///     installed.
///
/// - [`product_language`](https://learn.microsoft.com/en-us/windows/win32/msi/productlanguage)
///     Specifies the language the installer should use for any strings in the
///     user interface that are not authored into the database. This property
///     must be a numeric language identifier.
///     - TODO: Figure out where to find the official definitions of these for
///       users. I believe English is 1033.
///
/// - [`product_code`](https://learn.microsoft.com/en-us/windows/win32/msi/productcode)
///     A unique identifier for the particular product release, represented as a
///     string GUID. This ID must vary for different versions and languages. Set
///     this to `*` to have the program generate the GUID automatically.
///
#[derive(Deserialize)]
#[serde(rename = "product_info")]
pub(crate) struct ProductInformationProperties {
    pub(crate) product_name: String,
    pub(crate) product_version: String,
    pub(crate) manufacturer: String,
    pub(crate) product_language: u16,
    pub(crate) product_code: String,
}

/// # [Summary Information Properties](https://learn.microsoft.com/en-us/windows/win32/msi/summary-property-descriptions)
///
/// There are several required properties in this section.
///
/// ## Properties
///
/// ### Required
///
/// - [`page_count`](https://learn.microsoft.com/en-us/windows/win32/msi/page-count-summary)
///     Contains the minimum installer version required by the installation
///     package.
///
/// - [`revision_number`](https://learn.microsoft.com/en-us/windows/win32/msi/revision-number-summary)
///     Contains the package code (GUID) for the installer package.
///     - TODO: How does this relate to the product_code GUID?
///     - TODO: Can this be automatically generated? If it can add a note to
///       this comment section saying so.
///
/// - [`template`](https://learn.microsoft.com/en-us/windows/win32/msi/template-summary)
///     The platform and languages compatible with this installation package.
///
/// - [`word_count`](https://learn.microsoft.com/en-us/windows/win32/msi/word-count-summary)
///     The type of the source file image.
///
/// ### Optional
///
/// - [`author`](https://learn.microsoft.com/en-us/windows/win32/msi/author-summary)
///     The name of the author publishing the installation package, transform,
///     or patch package.
///
/// - [`code_page`](https://learn.microsoft.com/en-us/windows/win32/msi/codepage-summary)
///     The numeric value of the ANSI code page used for any strings that are
///     stored in the summary information
///
/// - [`comments`](https://learn.microsoft.com/en-us/windows/win32/msi/comments-summary)
///     Conveys the general purpose of the installation package, transform, or
///     patch package.
///
/// - [`generating_application`](https://learn.microsoft.com/en-us/windows/win32/msi/creating-application-summary)
///     Contains the name of the software used to author this MSI. If this is
///     not set in the config, it is populated with "MSI Builder".
///
#[derive(Deserialize)]
#[serde(rename = "summary_info")]
pub(crate) struct SummaryInformationProperties {
    // Required
    pub(crate) page_count: u16,
    pub(crate) revision_number: String,
    pub(crate) template: String,
    // Optional in config, required by MSI.
    pub(crate) word_count: Option<u16>,
    // Optional
    pub(crate) author: Option<String>,
    pub(crate) code_page: Option<String>,
    pub(crate) comments: Option<String>,
    pub(crate) generating_application: Option<String>,
}

/// # Default Files
///
/// These are properties that are used to quickly define where source files are
/// to be placed on the target system.
///
/// ## Properties
///
/// - *explicit_paths* If true, the user needs to specify full paths for install
///   folders. If false, several filepaths are autocompleted for brevity.
///   Defaults to false if not specified.
///     - Specifying `program_files` under the `default_files` header will copy
///       the contents of the listed directory to `C:\Program
///       Files\[Manufacturer]\[ProductName]\`.
///     - Specifying `program_files_32` under the `default_files` header will
///       copy the contents of the listed directory to `C:\Program Files
///       (x86)\[Manufacturer]\[ProductName]\`.
///     - Specifying `desktop` under the `default_files` header will copy the
///       contents of the listed directory to `C:\Users\[Username]\[Desktop]\`.
///
#[derive(Deserialize)]
#[serde(rename = "default_files")]
pub(crate) struct DefaultFiles {
    pub(crate) program_files: Option<String>,
    pub(crate) program_files_32: Option<String>,
    pub(crate) desktop: Option<String>,
}

/// # Explicit Files
///
/// These are properties that are used to give the user more fine grain control
/// of where source files are placed on the target system.
///
/// ## Properties
///
///
#[derive(Deserialize)]
#[serde(rename = "explicit_files")]
pub(crate) struct ExplicitFiles {}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONFIG: &str = r#"
[product_info]
product_name = "Test Application"
product_version = "22.1.15"
manufacturer = "Myself"
product_language = 1033
product_code = "*"

[summary_info]
page_count = 200
revision_number = "*"
template = "x64;1033"
author = "Test Name"
"#;

    #[test]
    fn config_deserializes() {
        let c: MsiConfig = toml::from_str(TEST_CONFIG).unwrap();

        // Product Info Properties
        assert_eq!(c.product_info.product_name, "Test Application");
        assert_eq!(c.product_info.product_version, "22.1.15");
        assert_eq!(c.product_info.manufacturer, "Myself");
        assert_eq!(c.product_info.product_language, 1033);
        assert_eq!(c.product_info.product_code, "*");

        // Summary Info Properties
        assert_eq!(c.summary_info.page_count, 200);
        assert_eq!(c.summary_info.revision_number, "*");
        assert_eq!(c.summary_info.template, "x64;1033");
        assert!(c.summary_info.author.is_some());
        assert_eq!(c.summary_info.author.unwrap(), "Test Name");
    }
}
