use std::{fs::File, rc::Rc};

use cli_table::{Cell, CellStruct, Style, Table};
use log::{debug, error, info};
use msi::{Package, Select, Value};

use crate::AllowedToList as ATL;

pub(crate) fn list(input_file: String, list_item: ATL) -> Result<String, ()> {
    info!("Reading MSI {}", input_file);
    let mut msi = match msi::open_rw(input_file) {
        Ok(msi) => msi,
        Err(_) => return Err(()),
    };

    match list_item {
        ATL::Author => list_author(msi),
        ATL::Tables => list_tables(msi),
        ATL::TableColumns { table } => list_table_columns(msi, table),
        ATL::TableContents { table } => list_table_contents(&mut msi, table),
    }
}

fn list_author(msi: Package<File>) -> Result<String, ()> {
    debug!("Listing author of MSI");
    let author = msi.summary_info().author().unwrap_or_default();
    Ok(author.to_owned())
}

fn list_tables(msi: Package<File>) -> Result<String, ()> {
    debug!("Listing tables in MSI");
    let tables = msi.tables().map(|t| t.name()).collect::<Vec<&str>>();
    Ok(tables.join("\n"))
}

fn list_table_columns(msi: Package<File>, table: String) -> Result<String, ()> {
    debug!("Listing the columns of table {} in MSI", table);
    let Some(table) = msi.get_table(&table) else {
        error!("Table {} could not be found in MSI", table);
        return Err(());
    };
    let columns = table
        .columns()
        .iter()
        .map(|c| c.name())
        .collect::<Vec<&str>>();
    Ok(columns.join("\n"))
}

fn list_table_contents(msi: &mut Package<File>, table_name: String) -> Result<String, ()> {
    debug!("Listing the contents of table {} in MSI", table_name);

    let rows = match msi.select_rows(Select::table(&table_name)) {
        Ok(rows) => rows,
        Err(_) => {
            error!("Failed to get rows from table {}", table_name);
            return Err(());
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
