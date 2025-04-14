#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::{rc::Rc, str::FromStr};

use camino::Utf8PathBuf;
use flexstr::{local_str, LocalStr};
use itertools::{Either, Itertools};
use msi::{Category, Column, Insert, Value};
use uuid::Uuid;

use crate::{
    builder::Msi,
    config::MsiConfig,
    error,
    helpers::{debug, warns},
    info,
    models::{
        directory::Directory, error::MsiError, file::File, sequencer::Sequencer,
    },
    traits::identifier::Identifier,
};

const DOT: LocalStr = local_str!(".");
const SOURCEDIR: LocalStr = local_str!("SourceDir");
const TARGETDIR: LocalStr = local_str!("TARGETDIR");
const PROGRAMFILESFOLDER: LocalStr = local_str!("ProgramFilesFolder");
const PROGRAMFILES64FOLDER: LocalStr = local_str!("ProgramFiles64Folder");

pub(crate) fn add_paths(
    package: &mut Msi,
    config: Rc<MsiConfig>,
    input_directory: &Utf8PathBuf,
) -> Result<(), MsiError> {
    info!("Getting paths to include in the MSI");
    let (directories, _fields) = scan_paths(config, input_directory)?;

    create_directory_table(package)?;

    let query = Insert::into("Directory").rows(
        directories
            .iter()
            .map(|dir| {
                vec![
                    Value::from(dir.id().to_string()),
                    match &dir.parent_id() {
                        Some(p) => Value::from(p.to_string()),
                        None => Value::Null,
                    },
                    Value::from(dir.name().to_string()),
                ]
            })
            .collect(),
    );

    if let Err(err) = package.insert_rows(query) {
        return Err(MsiError::nested("Failed to insert row into table", err));
    };

    Ok(())
}

fn scan_paths(
    config: Rc<MsiConfig>,
    input_directory: &Utf8PathBuf,
) -> Result<(Vec<Directory>, Vec<File>), MsiError> {
    // Keeps track of the file installation order. The `File` object has a
    // sequence field that needs to be
    let mut file_sequencer = Sequencer::new(1);
    let mut directories = Vec::new();
    let mut files = Vec::new();

    if config.explicit_files.is_some() {
        let (scanned_dirs, scanned_files) = &mut add_explicit_path_directories(
            config.clone(),
            input_directory,
            &mut file_sequencer,
        )?;
        directories.append(scanned_dirs);
        files.append(scanned_files);
    }

    if config.default_files.is_some() {
        let (scanned_dirs, scanned_files) = &mut add_default_directories(
            config,
            input_directory,
            &mut file_sequencer,
        )?;
        directories.append(scanned_dirs);
        files.append(scanned_files);
    }

    Ok((directories, files))
}

fn add_explicit_path_directories(
    _config: Rc<MsiConfig>,
    _input_directory: &Utf8PathBuf,
    _file_sequencer: &mut Sequencer,
) -> Result<(Vec<Directory>, Vec<File>), MsiError> {
    warns!("Sorry! Explicit paths are currently not implemented.");
    // TODO: Finish implementing explicit path directories.
    todo!("Explicit paths are currently not supported.");
}

fn add_default_directories(
    config: Rc<MsiConfig>,
    input_directory: &Utf8PathBuf,
    file_sequencer: &mut Sequencer,
) -> Result<(Vec<Directory>, Vec<File>), MsiError> {
    debug!(
        "Adding default directories for input path [{}]",
        input_directory
    );
    let files_section = config
        .default_files
        .as_ref()
        .expect("Failed to get `default_files` section from MsiConfig");

    let mut default_directories = vec![
        // The value of the DefaultDir column for the root directory entry must
        // be set to the SourceDir property per [this
        // section](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table#root-source-directory).
        // This will be present in every install with a files section.
        Directory::new(TARGETDIR, None, SOURCEDIR, None),
    ];

    // Add the Program Files listing if it is included in the config.
    if let Some(program_files) = &files_section.program_files {
        default_directories.append(&mut program_files_directory(
            &config,
            PROGRAMFILES64FOLDER,
            input_directory.join(Utf8PathBuf::from_str(program_files).unwrap()),
        ));
    };

    // Add the Program Files (x86) listing if it is included in the config.
    if let Some(program_files_32) = &files_section.program_files_32 {
        default_directories.append(&mut program_files_directory(
            &config,
            PROGRAMFILESFOLDER,
            input_directory
                .join(Utf8PathBuf::from_str(program_files_32).unwrap()),
        ));
    };

    // Add the Desktop listing if it is included in the config.
    if let Some(desktop) = &files_section.desktop {
        default_directories.push(Directory::new(
            "DesktopFolder".to_string(),
            Some(TARGETDIR),
            DOT,
            Some(input_directory.join(Utf8PathBuf::from_str(desktop).unwrap())),
        ));
    };

    let mut directories = default_directories.clone();
    let mut files = Vec::new();
    for directory in default_directories {
        let Some(path) = directory.source() else {
            debug!("Not scanning directory [{}] for paths", directory.name());
            continue;
        };
        // Scan the current path and append
        let (scanned_dirs, scanned_files) =
            &mut scan_path(path, file_sequencer, directory.id())?;
        directories.append(scanned_dirs);
        files.append(scanned_files);
    }

    Ok((directories, files))
}

fn create_directory_table(package: &mut Msi) -> Result<(), MsiError> {
    let result = package.create_table(
        "Directory",
        vec![
            Column::build("Directory").primary_key().id_string(72),
            Column::build("Directory_Parent").nullable().id_string(72),
            Column::build("DefaultDir")
                .category(Category::DefaultDir)
                .string(255),
        ],
    );

    if let Err(e) = result {
        let err = error!("Failed to create directory table: {}", e);
        return Err(MsiError::nested(err, Box::new(e)));
    }

    Ok(())
}

fn program_files_directory(
    config: &Rc<MsiConfig>,
    program_files_label: LocalStr,
    source_dir: Utf8PathBuf,
) -> Vec<Directory> {
    let program_folder_uuid = Uuid::as_identifier();
    let manufacturer_folder_uuid = Uuid::as_identifier();
    vec![
        Directory::new(program_files_label.clone(), Some(TARGETDIR), DOT, None),
        Directory::new(
            manufacturer_folder_uuid.clone(),
            Some(program_files_label),
            config.product_info.manufacturer.to_string(),
            None,
        ),
        Directory::new(
            program_folder_uuid,
            Some(manufacturer_folder_uuid),
            config.product_info.product_name.to_string(),
            Some(source_dir),
        ),
    ]
}

fn scan_path(
    scan_target: &Utf8PathBuf,
    sequencer: &mut Sequencer,
    parent_directory_id: &str,
) -> Result<(Vec<Directory>, Vec<File>), MsiError> {
    debug!("Scanning directory path [{}]", scan_target);
    // Get the entries present in the `scan_target` directory.
    let directory_entries = match scan_target.read_dir_utf8() {
        Ok(dir) => dir,
        Err(e) => {
            return Err(MsiError::nested(
                format!("Failed to read directory [{}]", scan_target),
                e,
            ));
        }
    };

    // Get all of the entries that did not return an `Err` when scanned.
    let (ok_entries, errs): (Vec<_>, Vec<_>) =
        directory_entries.partition_result();
    // If any of them returned an error, short circuit and return that error.
    // May change this behavior based on config if desired in the future.
    if let Some(err) = errs.first() {
        // @GrimOutlook for future reference, DO NOT try to just pass `err` into
        // the MsiError without reconstructing it into a new error. I just spent
        // an hour trying to figure out why `err` gets dropped when doing this
        // but I'm far too stupid it seems. Rust forums seem to indicate maybe
        // std::io::Error stores a reference that gets dropped and it's lifetime
        // is held to that but idk. Let someone smarter tell you the way to fix
        // it because I guarantee it's simple but I cannot find it for the life
        // of me. And don't start thinking that you just need to call `.clone()`
        // on it. `std::io:Error` doesn't implement `Clone` and I tried cloning
        // everything else (attempting to flail my way into an answer) to no
        // avail.
        return Err(MsiError::nested(
            error!("Failed to read file inside {}", scan_target),
            std::io::Error::new(err.kind(), err.to_string()),
        ));
    }

    // Get all of the entries that have a valid file type. We need to check if
    // these are directories so if we can't read that from somewhere we need to
    // exit.
    //
    // Also I shamelessly stole the [implementation] for
    // [`partition_result`](https://docs.rs/itertools/0.14.0/src/itertools/lib.rs.html#3669-3679)
    // in itertools to make this because I needed the entries back out, not the
    // filetypes that have to be checked.
    let (ok_type_entries, errs): (Vec<_>, Vec<_>) =
        ok_entries.iter().partition_map(|d| match d.file_type() {
            Ok(filetype) => Either::Left((d, filetype)),
            Err(e) => Either::Right(e),
        });
    // Short circuit if any of the filetypes were unable to be read.
    //
    // The block of text above about `std::io::Error`also applies here. Just
    // don't think about it, it's fine.
    if let Some(err) = errs.first() {
        return Err(MsiError::nested(
            error!("Failed to read file type from file inside {}", scan_target),
            std::io::Error::new(err.kind(), err.to_string()),
        ));
    }

    // Convert all of the entries that are directories into PathBufs for later
    // use.
    let (found_dir_paths, found_file_paths): (Vec<_>, Vec<_>) = ok_type_entries
        .into_iter()
        // Only keep the entries that are either directories or files. We don't
        // care about symlinks or other file types.
        .filter(|(_entry, filetype)| filetype.is_dir() || filetype.is_file())
        .map(|(entry, filetype)| (entry.path(), filetype))
        .partition_map(|(entry, filetype)| {
            if filetype.is_dir() {
                Either::Left(entry.to_path_buf())
            } else {
                Either::Right(entry.to_path_buf())
            }
        });

    // Convert all of the found directories found in the scan_path directory to
    // Directory objects. We need to generate a UUID for them and have those
    // available to pass into the recursive `scan_path` call so they know what
    // parent directory they are related to
    //
    // We only have to do this because we return Directory objects instead of
    // just PathBuf objects. TODO: Look into only returning PathBuf objects and
    // converting them outside of this function. I'm hesitant this will be much
    // cleaner because I feel like I'll just have to remake the structure
    // already present here but required more thought.
    let found_directories = found_dir_paths
        .iter()
        .map(|source| Directory::from_path(source, parent_directory_id))
        .collect_vec();

    // Recursively scan all of the directories that were found in the
    // `scan_target` directory and return all of the files and directories that
    // were found.
    //
    // There has to be a better way than making `errs` mutable and popping it
    // out but if I try to do `errs.first()`but that returns a reference and I
    // couldn't figure out how to get around the `cannot move out of `*err`
    // which is behind a shared reference`.
    let (mut all_dirs, mut all_files): (Vec<Directory>, Vec<File>) =
        (Default::default(), Default::default());
    let path_scan_results = found_directories
        .iter()
        .map(|dir| {
            all_dirs.push(dir.clone());
            scan_path(dir.source().as_ref().unwrap(), sequencer, dir.id())
        })
        .collect_vec();
    for paths in path_scan_results {
        match paths {
            Ok(paths) => {
                let (mut found_dirs, mut found_files) = paths;
                all_dirs.append(&mut found_dirs);
                all_files.append(&mut found_files);
            }
            // Short circuit if any errors were hit during the recursive scan.
            Err(err) => return Err(err),
        }
    }

    for file_path in found_file_paths {
        let size = match file_path.metadata() {
            Ok(metadata) => {
                #[cfg(target_os = "linux")]
                {
                    metadata.size()
                }
                #[cfg(target_os = "windows")]
                {
                    metadata.file_size()
                }
            }
            Err(err) => {
                return Err(MsiError::nested(
                    error!("Couldn't get metadata from file [{}]", file_path),
                    err,
                ))
            }
        };
        let file = File::new(&file_path, sequencer.get(), size);
        all_files.push(file);
    }

    Ok((all_dirs, all_files))
}
