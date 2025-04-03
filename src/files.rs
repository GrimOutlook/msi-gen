use std::{fs::FileType, rc::Rc};

use camino::Utf8PathBuf;
use itertools::{Either, Itertools};
use msi::{Category, Column, Insert, Value};

use crate::{
    builder::Msi,
    config::MsiConfig,
    error,
    models::{directory::Directory, error::MsiError},
};

pub(crate) fn add_files(
    package: &mut Msi,
    config: Rc<MsiConfig>,
    input_directory: &Utf8PathBuf,
) -> Result<(), MsiError> {
    let directories = add_directories(config, input_directory)?;

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

fn add_directories(
    config: Rc<MsiConfig>,
    input_directory: &Utf8PathBuf,
) -> Result<Vec<Directory>, MsiError> {
    let mut directories = Vec::new();
    if config.explicit_files.is_some() {
        directories.append(&mut add_explicit_path_directories(
            config.clone(),
            input_directory,
        )?);
    }
    if config.default_files.is_some() {
        directories.append(&mut add_default_directories(config, input_directory)?);
    }

    Ok(directories)
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

    let (program_files_label, source_dir_raw) = match &files_section.program_files {
        Some(program_files) => (
            "ProgramFiles64Folder".to_string(),
            program_files.to_string(),
        ),
        None => (
            "ProgramFilesFolder".to_string(),
            files_section
                .program_files_32
                .as_ref()
                // This check should already be handled by the `check_config`
                // function in `builder.rs`
                .expect("Parsed `[default_files]` section incorrectly causing an unexpected panic.")
                .to_string(),
        ),
    };

    // Convert the string representing the path to scan into a path.
    let source_dir =
        match camino::absolute_utf8(input_directory.join(Utf8PathBuf::from(&source_dir_raw))) {
            Ok(dir) => dir,
            Err(e) => {
                return Err(MsiError::nested(
                    format!("Failed to get full path of directory [{}]", source_dir_raw),
                    e,
                ))
            }
        };

    if !source_dir.exists() {
        return Err(MsiError::short(format!(
            "Directory [{}] listed for [{}] does not exist",
            source_dir, source_dir_raw
        )));
    }

    if !source_dir.is_dir() {
        return Err(MsiError::short(format!(
            "Path [{}] listed for [{}] is not a directory",
            source_dir, source_dir_raw
        )));
    }

    let mut dirs = vec![
        // The value of the DefaultDir column for the root directory entry must
        // be set to the SourceDir property per [this
        // section](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table#root-source-directory)
        Directory {
            id: "TARGETDIR".to_string(),
            parent: None,
            name: "SourceDir".to_string(),
        },
        Directory {
            id: program_files_label.clone(),
            parent: Some("TARGETDIR".to_string()),
            name: ".".to_string(),
        },
        Directory {
            id: "ManufacturerFolder".to_string(),
            parent: Some(program_files_label),
            name: config.product_info.manufacturer.to_string(),
        },
        Directory {
            id: "INSTALLDIR".to_string(),
            parent: Some("ManufacturerFolder".to_string()),
            name: config.product_info.product_name.to_string(),
        },
    ];

    let mut scanned_directories = scan_directories(&config, &source_dir)?;
    dirs.append(&mut scanned_directories);

    Ok(dirs)
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

fn scan_directories(
    config: &Rc<MsiConfig>,
    scan_target: &Utf8PathBuf,
) -> Result<Vec<Directory>, MsiError> {
    // Get the listings present in the `scan_target` directory.
    let directory_listings = match scan_target.read_dir_utf8() {
        Ok(dir) => dir,
        Err(e) => {
            return Err(MsiError::nested(
                format!("Failed to read directory [{}]", scan_target),
                e,
            ));
        }
    };

    // Get all of the listings that did not return an `Err` when scanned.
    let (ok_listings, errs): (Vec<_>, Vec<_>) = directory_listings.partition_result();
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

    // Get all of the listings that have a valid file type. We need to check if
    // these are directories so if we can't read that from somewhere we need to
    // exit.
    //
    // Also I shamelessly stole the [implementation] for
    // [`partition_result`](https://docs.rs/itertools/0.14.0/src/itertools/lib.rs.html#3669-3679)
    // in itertools to make this because I needed the listings back out, not the
    // filetypes that have to be checked.
    let (ok_type_listings, errs): (Vec<_>, Vec<_>) =
        ok_listings.iter().partition_map(|d| match d.file_type() {
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

    // Convert all of the listings that are directories into PathBufs for later
    // use.
    let ok_dirs: Vec<_> = ok_type_listings
        .iter()
        .filter(|&(_listing, filetype)| filetype.is_dir())
        .map(|(listing, _filetype)| listing.path())
        .collect();

    let (ok_subdirs, errs): (Vec<_>, Vec<_>) = ok_dirs
        .iter()
        .map(|d| scan_directories(config, &Utf8PathBuf::from(d)))
        .partition_result();

    todo!("Finish")
}
