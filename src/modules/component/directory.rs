use camino::Utf8PathBuf;
use derive_new::new;
use flexstr::LocalStr;
use getset::Getters;
use uuid::Uuid;

use crate::modules::traits::identifier::Identifier;

/// # [Directory](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table)
///
/// This structure tracks directories that are created and interacted with by
/// the installing MSI.
///
/// ## Properties
///
/// - `id` A unique identifier for a directory or directory path.
/// - `parent_id` The ID of the directory that contains this directory. This is
///   a string and not a `PathBuf` because files can have a property based
///   parent such as `ProgramFiles`, `Desktop`, or `TARGETDIR`.
/// - `name` What the directory will be named (localizable) on the target
///   system.
/// - `source` Path to this directory on the system when generating the MSI.
///   This is optional because some of the default paths do not need to specify
///   a source, such as `DesktopFolder` and `ProgramFiles`, they are simply used
///   in the hierarchy.
#[derive(Clone, Debug, Getters, new)]
#[getset(get = "pub")]
pub(crate) struct Directory {
    #[new(into)]
    id: LocalStr,
    #[new(into)]
    parent_id: Option<LocalStr>,
    #[new(into)]
    name: LocalStr,
    source: Option<Utf8PathBuf>,
}

impl Directory {
    pub fn from_path(source: &Utf8PathBuf, parent_id: &str) -> Self {
        Directory {
            id: Uuid::as_identifier(),
            parent_id: Some(parent_id.into()),
            name: source
                .file_name()
                .expect("Filename somehow ends with '..'. Ending in pure confusion.")
                .into(),
            source: Some(source.clone()),
        }
    }
}
