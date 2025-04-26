use flexstr::LocalStr;
use serde::Deserialize;

/// # [Summary Information Properties](https://learn.microsoft.com/en-us/windows/win32/msi/summary-property-descriptions)
///
/// There are several required properties in this section.
///
/// ## Properties
///
/// ### Required
///
/// - [`page_count`](https://learn.microsoft.com/en-us/windows/win32/msi/page-count-summary)
///   Contains the minimum installer version required by the installation
///   package.
///
/// - [`revision_number`](https://learn.microsoft.com/en-us/windows/win32/msi/revision-number-summary)
///   Contains the package code (GUID) for the installer package.
///     - TODO: How does this relate to the product_code GUID?
///     - TODO: Can this be automatically generated? If it can add a note to
///       this comment section saying so.
///
/// - [`template`](https://learn.microsoft.com/en-us/windows/win32/msi/template-summary)
///   The platform and languages compatible with this installation package.
///
/// - [`word_count`](https://learn.microsoft.com/en-us/windows/win32/msi/word-count-summary)
///   The type of the source file image.
///
/// ### Optional
///
/// - [`author`](https://learn.microsoft.com/en-us/windows/win32/msi/author-summary)
///   The name of the author publishing the installation package, transform, or
///   patch package.
///
/// - [`code_page`](https://learn.microsoft.com/en-us/windows/win32/msi/codepage-summary)
///   The numeric value of the ANSI code page used for any strings that are
///   stored in the summary information
///
/// - [`comments`](https://learn.microsoft.com/en-us/windows/win32/msi/comments-summary)
///   Conveys the general purpose of the installation package, transform, or
///   patch package.
///
/// - [`generating_application`](https://learn.microsoft.com/en-us/windows/win32/msi/creating-application-summary)
///   Contains the name of the software used to author this MSI. If this is not
///   set in the config, it is populated with "whimsi".
///
#[derive(Deserialize)]
#[serde(rename = "summary_info")]
pub(crate) struct SummaryInformationProperties {
    // Required
    pub(crate) page_count: u16,
    pub(crate) revision_number: LocalStr,
    pub(crate) template: LocalStr,
    // Optional in config, required by MSI.
    pub(crate) word_count: Option<u16>,
    // Optional
    pub(crate) author: Option<String>,
    pub(crate) code_page: Option<String>,
    pub(crate) comments: Option<String>,
    pub(crate) generating_application: Option<String>,
}
