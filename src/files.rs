use std::{os::unix::fs::MetadataExt, rc::Rc};

use camino::Utf8PathBuf;
use itertools::{Either, Itertools};
use msi::{Category, Column, Insert, Value};
use uuid::Uuid;

use crate::{
    builder::Msi,
    config::MsiConfig,
    error,
    models::{directory::Directory, error::MsiError, file::File, sequencer::Sequencer},
};

pub(crate) fn add_paths(
    package: &mut Msi,
    config: Rc<MsiConfig>,
    input_directory: &Utf8PathBuf,
) -> Result<(), MsiError> {
    let (directories, fields) = scan_paths(config, input_directory)?;

    create_directory_table(package)?;

    let query = Insert::into("Directory").rows(
        directories
            .iter()
            .map(|dir| {
                vec![
                    Value::from(dir.id.clone()),
                    match &dir.parent {
                        Some(p) => Value::from(p.to_string()),
                        None => Value::Null,
                    },
                    Value::from(dir.name.clone()),
                ]
            })
            .collect(),
    );

    package
        .insert_rows(query)
        .expect("Failed to add directory rows to MSI");

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
        let explicit_dirs = &mut add_explicit_path_directories(config.clone(), input_directory)?;
        directories.append(explicit_dirs);
    }

    if config.default_files.is_some() {
        let default_dirs = &mut add_default_directories(config, input_directory)?;
        directories.append(default_dirs);
    }

    let (scanned_dirs, scanned_files) =
        &mut scan_path(&Utf8PathBuf::from(input_directory), &mut file_sequencer)?;

    directories.append(scanned_dirs);
    files.append(scanned_files);

    Ok((directories, files))
}

fn add_explicit_path_directories(
    _config: Rc<MsiConfig>,
    _input_directory: &Utf8PathBuf,
) -> Result<Vec<Directory>, MsiError> {
    // TODO: Finish implementing explicit path directories.
    todo!("Explicit paths are currently not supported.");
}

fn add_default_directories(
    config: Rc<MsiConfig>,
    input_directory: &Utf8PathBuf,
) -> Result<Vec<Directory>, MsiError> {
    let files_section = config
        .default_files
        .as_ref()
        .expect("Failed to get `default_files` section from MsiConfig");

    let mut directories = vec![
        // The value of the DefaultDir column for the root directory entry must
        // be set to the SourceDir property per [this
        // section](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table#root-source-directory).
        // This will be present in every install with a files section.
        Directory {
            id: "TARGETDIR".to_string(),
            parent: None,
            name: "SourceDir".to_string(),
            source: None,
        },
    ];

    // Add the Program Files listing if it is included in the config.
    if let Some(program_files) = &files_section.program_files {
        directories.append(&mut program_files_directory(
            &config,
            "ProgramFiles64Folder".to_string(),
            input_directory.join(program_files),
        ));
    };

    // Add the Program Files (x86) listing if it is included in the config.
    if let Some(program_files_32) = &files_section.program_files_32 {
        directories.append(&mut program_files_directory(
            &config,
            "ProgramFilesFolder".to_string(),
            input_directory.join(program_files_32),
        ));
    };

    // Add the Desktop listing if it is included in the config.
    if let Some(desktop) = &files_section.desktop {
        directories.append(&mut program_files_directory(
            &config,
            "DesktopFolder".to_string(),
            input_directory.join(desktop),
        ));
    };

    Ok(directories)
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
    program_files_label: String,
    source_dir: Utf8PathBuf,
) -> Vec<Directory> {
    let program_folder_uuid = Uuid::new_v4().to_string();
    let manufacturer_folder_uuid = Uuid::new_v4().to_string();
    vec![
        Directory {
            id: program_files_label.clone(),
            parent: Some("TARGETDIR".to_string()),
            name: ".".to_string(),
            source: None,
        },
        Directory {
            id: manufacturer_folder_uuid.clone(),
            parent: Some(program_files_label),
            name: config.product_info.manufacturer.to_string(),
            source: None,
        },
        Directory {
            id: program_folder_uuid,
            parent: Some(manufacturer_folder_uuid),
            name: config.product_info.product_name.to_string(),
            source: Some(source_dir),
        },
    ]
}

fn scan_path(
    scan_target: &Utf8PathBuf,
    sequencer: &mut Sequencer,
) -> Result<(Vec<Directory>, Vec<File>), MsiError> {
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
    let (ok_entries, errs): (Vec<_>, Vec<_>) = directory_entries.partition_result();
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
            error!("Failed to read file inside {}", scan_target),
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

    // Recursively scan all of the directories that were found in the
    // `scan_target` directory and return all of the files and directories that
    // were found.
    //
    // There has to be a better way than making `errs` mutable and popping it
    // out but if I try to do `errs.first()`but that returns a reference and I
    // couldn't figure out how to get around the `cannot move out of `*err`
    // which is behind a shared reference`.
    let (mut subdirs, mut subdir_files): (Vec<Directory>, Vec<File>) =
        (Default::default(), Default::default());
    for paths in found_dir_paths
        .iter()
        .map(|d| scan_path(&Utf8PathBuf::from(d), sequencer))
    {
        match paths {
            Ok(paths) => {
                let (mut found_dirs, mut found_files) = paths;
                subdirs.append(&mut found_dirs);
                subdir_files.append(&mut found_files);
            }
            // Short circuit if any errors were hit during the recursive scan.
            Err(err) => return Err(err),
        }
    }

    subdirs.append(&mut found_dir_paths.iter().map_into().collect_vec());

    for file_path in found_file_paths {
        let size = match file_path.metadata() {
            Ok(metadata) => metadata.size(),
            Err(err) => {
                return Err(MsiError::nested(
                    error!("Couldn't get metadata from file [{}]", file_path),
                    err,
                ))
            }
        };
        let file = File::new(&file_path, sequencer.get(), size);
        subdir_files.push(file);
    }

    Ok((subdirs, subdir_files))
}
