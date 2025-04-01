/// # [Directory](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table)
///
/// This structure tracks directories that are created and interacted with by
/// the installing MSI.
///
/// ## Properties
///
/// - `id` A unique identifier for a directory or directory path.
/// - `parent` A reference to the directory's parent directory.
/// - `name` The directory's name (localizable) under the parent directory.
pub(crate) struct Directory {
    pub(crate) id: String,
    pub(crate) parent: Option<String>,
    pub(crate) name: String,
}
