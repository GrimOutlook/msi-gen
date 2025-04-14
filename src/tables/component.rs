// Populates the `File` table

use msi::{Category, Column};

use crate::{
    builder::Msi,
    error,
    models::{error::MsiError, file::File},
};

const TABLE_NAME: &str = "Component";

pub fn populate_component_table(
    package: &mut Msi,
    files: &Vec<File>,
) -> Result<(), MsiError> {
    create_component_table(package)?;

    let query = Insert::into(TABLE_NAME).rows(
        files
            .iter()
            .map(|file| {
                vec![
                    Value::from(file.component_id().to_string()),
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

fn create_component_table(package: &mut Msi) -> Result<(), MsiError> {
    let result = package.create_table(
        TABLE_NAME,
        vec![
            Column::build("Component").primary_key().id_string(72),
            Column::build("ComponentId")
                .category(Category::Guid)
                .nullable()
                .string(38),
            Column::build("Directory_").id_string(72),
            Column::build("Attributes").int16(),
            Column::build("Condition")
                .nullable()
                .category(Category::Condition)
                .string(255),
            Column::build("KeyPath").nullable().id_string(72),
        ],
    );

    if let Err(e) = result {
        let err = error!("Failed to create {} table: {}", TABLE_NAME, e);
        return Err(MsiError::nested(err, Box::new(e)));
    }

    Ok(())
}
