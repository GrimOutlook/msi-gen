// Populates the `Directory` table

use msi::{Category, Column, Insert, Value};

use crate::{
    builder::Msi,
    error,
    models::{directory::Directory, error::MsiError},
};

pub fn populate_directory_table(
    package: &mut Msi,
    directories: &Vec<Directory>,
) -> Result<(), MsiError> {
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
        let err = error!("Failed to create Directory table: {}", e);
        return Err(MsiError::nested(err, Box::new(e)));
    }

    Ok(())
}
