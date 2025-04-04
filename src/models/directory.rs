use camino::Utf8PathBuf;

/// # [Directory](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table)
///
/// This structure tracks directories that are created and interacted with by
/// the installing MSI.
///
/// ## Properties
///
/// - `id` A unique identifier for a directory or directory path.
/// - `parent` A reference to the directory's parent directory. This is a string
///   and not a `PathBuf` because files can have a property based parent such as
///   `ProgramFiles`, `Desktop`, or `TARGETDIR`.
/// - `name` The directory's name (localizable) under the parent directory.
/// - `source` Path to this directory on the system when generating the MSI.
///   This is optional because some of the default paths do not need to specify
///   a source, they are simply used in the hierarchy.
pub(crate) struct Directory {
    pub(crate) id: String,
    pub(crate) parent: Option<String>,
    pub(crate) name: String,
    pub(crate) source: Option<Utf8PathBuf>,
}

impl From<&Utf8PathBuf> for Directory {
    fn from(value: &Utf8PathBuf) -> Self {
        todo!("Implement conversion from path buf to directory")
    }
}
