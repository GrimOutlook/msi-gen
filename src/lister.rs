use cli_table::{Cell, CellStruct, Style, Table};
use msi::{Package, Select};
use std::fs::File;

use crate::helpers::{debug, error, info};

use crate::{models::error::MsiError, AllowedToList as ATL};

pub(crate) fn list(input_file: String, list_item: ATL) -> Result<String, MsiError> {
    info!("Reading MSI {}", input_file);
    let mut msi = match msi::open_rw(input_file) {
        Ok(msi) => msi,
        Err(e) => {
            let msg = error!("Failed to open MSI");
            return Err(MsiError::nested(msg, e));
        }
    };

    match list_item {
        ATL::Author => list_author(msi),
        ATL::Tables => list_tables(msi),
        ATL::TableColumns { table } => list_table_columns(msi, table),
        ATL::TableContents { table } => list_table_contents(&mut msi, table),
    }
}

fn list_author(msi: Package<File>) -> Result<String, MsiError> {
    debug!("Listing author of MSI");
    let author = msi.summary_info().author().unwrap_or_default();
    Ok(author.to_owned())
}

fn list_tables(msi: Package<File>) -> Result<String, MsiError> {
    debug!("Listing tables in MSI");
    let tables = msi.tables().map(|t| t.name()).collect::<Vec<&str>>();
    Ok(tables.join("\n"))
}

/// List the columns present in the given table
fn list_table_columns(msi: Package<File>, table: String) -> Result<String, MsiError> {
    debug!("Listing the columns of table {} in MSI", table);
    let table = match msi.get_table(&table) {
        Some(table) => table,
        None => {
            let err = error!("Table {} could not be found in MSI", table);
            return Err(MsiError::short(err));
        }
    };

    let columns = table.columns();

    let contents: Vec<Vec<CellStruct>> = columns
        .iter()
        .map(|c| {
            vec![
                c.name().cell(),
                c.coltype().to_string().cell(),
                format!("{:?}", c.category()).cell(),
            ]
        })
        .collect();

    let table_columns = ["Column", "Type", "Category"];

    let print_table = contents
        .table()
        .title(table_columns.iter().map(|c| c.cell().bold(true)))
        .bold(true);

    Ok(print_table
        .display()
        .expect("Failed to display table")
        .to_string())
}

/// List the contents of the given table
fn list_table_contents(msi: &mut Package<File>, table_name: String) -> Result<String, MsiError> {
    debug!("Listing the contents of table {} in MSI", table_name);

    let rows = match msi.select_rows(Select::table(&table_name)) {
        Ok(rows) => rows,
        Err(e) => {
            let err = error!("Failed to get rows from table {}", table_name);
            return Err(MsiError::nested(err, Box::new(e)));
        }
    };

    let columns = rows
        .columns()
        .iter()
        .map(|c| c.name().to_string())
        .collect::<Vec<String>>();

    let contents: Vec<Vec<CellStruct>> = rows
        .map(|r| {
            columns
                .iter()
                .map(|c| r[c.as_str()].to_string().cell())
                .collect()
        })
        .collect();

    let table = contents
        .table()
        .title(columns.iter().map(|c| c.cell().bold(true)))
        .bold(true);

    Ok(table
        .display()
        .expect("Failed to display table")
        .to_string())
}
