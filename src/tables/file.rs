// Populates the `File` table

use msi::{Category, Column};

use crate::{
    builder::Msi,
    error,
    models::{error::MsiError, file::File},
};

pub fn populate_file_table(
    package: &mut Msi,
    files: &Vec<File>,
) -> Result<(), MsiError> {
    create_file_table(package)?;
    unimplemented!();

    // let query = Insert::into("File").rows(
    //     files
    //         .iter()
    //         .map(|file| {
    //             vec![
    //                 Value::from(file.id().to_string()),
    //                 match &dir.parent_id() {
    //                     Some(p) => Value::from(p.to_string()),
    //                     None => Value::Null,
    //                 },
    //                 Value::from(dir.name().to_string()),
    //             ]
    //         })
    //         .collect(),
    // );

    // if let Err(err) = package.insert_rows(query) {
    //     return Err(MsiError::nested("Failed to insert row into table", err));
    // };

    Ok(())
}

fn create_file_table(package: &mut Msi) -> Result<(), MsiError> {
    let result = package.create_table(
        "File",
        vec![
            Column::build("File").primary_key().id_string(72),
            Column::build("Component_").id_string(72),
            Column::build("FileName")
                .category(Category::Filename)
                .string(255),
            Column::build("FileSize")
                .category(Category::DoubleInteger)
                .int16(),
            Column::build("Version")
                .nullable()
                .category(Category::Version)
                .string(72),
            Column::build("Language")
                .nullable()
                .category(Category::Language)
                .string(20),
            Column::build("Attributes").nullable().int16(),
            Column::build("Sequence").int16(),
        ],
    );

    if let Err(e) = result {
        let err = error!("Failed to create File table: {}", e);
        return Err(MsiError::nested(err, Box::new(e)));
    }

    Ok(())
}
