use std::rc::Rc;

use msi::{Category, Column, Insert, Value};

use crate::{builder::Msi, config::MsiConfig, models::directory::Directory};

pub(crate) fn add_files(
    package: &mut Msi,
    config: Rc<MsiConfig>,
    input_directory: &str,
) -> Result<(), ()> {
    let directories = add_directories(package, config, input_directory);

    Ok(())
}

fn add_directories(
    package: &mut Msi,
    config: Rc<MsiConfig>,
    input_directory: &str,
) -> Vec<Directory> {
    let explicit_paths = match &config.meta {
        Some(props) => props.explicit_paths.unwrap_or(false),
        None => false,
    };

    let dirs = if explicit_paths {
        add_explicit_path_directories(config, input_directory)
    } else {
        add_default_directories(config, input_directory)
    };

    package
        .create_table(
            "Directory",
            vec![
                Column::build("Directory")
                    .primary_key()
                    .category(Category::Identifier)
                    .id_string(72),
                Column::build("Directory_Parent")
                    .category(Category::Identifier)
                    .nullable()
                    .id_string(72),
                Column::build("DefaultDir")
                    .category(Category::DefaultDir)
                    .id_string(255),
            ],
        )
        .expect("Failed to create Directory table for MSI");

    let query = Insert::into("Directory").rows(
        dirs.iter()
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

    dirs
}

fn add_explicit_path_directories(config: Rc<MsiConfig>, input_directory: &str) -> Vec<Directory> {
    todo!("Add explicit path directories")
}

fn add_default_directories(config: Rc<MsiConfig>, input_directory: &str) -> Vec<Directory> {
    // Add TARGETDIR. This should be the only directory with no parent.
    let dirs = vec![
        Directory {
            id: "TARGETDIR".to_string(),
            parent: None,
            name: "SourceDir".to_string(),
        },
        Directory {
            id: "ProgramFilesFolder".to_string(),
            parent: Some("TARGETDIR".to_string()),
            name: ".".to_string(),
        },
        Directory {
            id: "INSTALLDIR".to_string(),
            parent: Some("ManufacturerFolder".to_string()),
            name: config.product_info.manufacturer.to_string(),
        },
    ];
    dirs
}
