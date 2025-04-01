/// # [File](https://learn.microsoft.com/en-us/windows/win32/msi/file-table)
///
/// Represents a file that is to be copied from the MSI to the target system.
///
/// ## Properties
///
/// - `component_id` Internal identifier of the component that controls this
///   file. Must correspond to a tracked component_id.
/// - `file_id` Internal identifier of the file for the MSI. This must be
///   unique. Must correspond to a tracked file_id.
/// - `name` Filename of the file when placed on the system.
/// - `source` Path to the file when generating the MSI. Must correspond to a
///   file present on the system during MSI generation.
/// - `vital` Whether the entire install should fail if this file fails to be
///   installed.
/// - `size` The size of the file in bytes. This must be a non-negative number.
/// - `version` This field is the version string for a versioned file. This
///   field is blank for non-versioned files.
/// - `language` A list of decimal language IDs separated by commas.
/// - `sequence` Sequence position of this file on the media images. This order
///   must correspond to the order of the files in the cabinet if the files are
///   compressed. The integers in this field must be equal or greater than 1.
pub(crate) struct File {
    component_id: String,
    file_id: String,
    source: String,
    pub(crate) name: String,
    pub(crate) vital: bool,
    pub(crate) version: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) sequence: usize,
}
