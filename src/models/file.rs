/// # [File](https://learn.microsoft.com/en-us/windows/win32/msi/file-table)
///
/// Represents a file that is to be copied from the MSI to the target system.
///
/// ## Properties
///
/// - `component_id` Internal identifier of the component that controls this
///   file.
/// - `id` Internal identifier of the file for the MSI. This must be unique.
/// - `name` Filename of the file when placed on the system.
/// - `source` Path to the file when generating the MSI.
/// - `vital` Whether the entire install should fail if this file fails to be
///   installed.
/// - `size` The size of the file in bytes. This must be a non-negative number.
/// - `version` This field is the version string for a versioned file. This
///   field is blank for non-versioned files.
/// - `language` A list of decimal language IDs separated by commas.
/// - `sequence` Sequence position of this file on the media images. This order
///   must correspond to the order of the files in the cabinet if the files are
///   compressed. The integers in this field must be equal or greater than 1.
struct File {
    component_id: String,
    id: String,
    name: String,
    source: String,
    vital: bool,
    version: Option<String>,
    language: Option<String>,
    sequence: usize,
}
