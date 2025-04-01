use std::rc::Rc;

use camino::Utf8PathBuf;
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
    config: Rc<MsiConfig>,
    input_directory: &Utf8PathBuf,
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

    let (program_files_label, source_dir) = match &files_section.program_files {
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

    let mut scanned_directories = scan_directories(config, input_directory)?;
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
    config: Rc<MsiConfig>,
    input_directory: &Utf8PathBuf,
) -> Result<Vec<Directory>, MsiError> {
    Ok(Vec::new())
}
