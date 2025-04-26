use anyhow::bail;
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use cli_table::{Cell, CellStruct, Style, Table};
use flexstr::SharedStr;
use log::debug;
use log::error;
use log::info;
use msi::{Package, Select};
use std::{fs::File, process::ExitCode};

use super::command_line::AllowedToList as ATL;

pub(crate) fn list(input_file: &Utf8PathBuf, list_item: ATL) -> ExitCode {
    info!("Reading MSI {}", input_file);

    let mut msi = match msi::open_rw(input_file)
        .with_context(|| "Failed to open {input_file}")
    {
        Ok(msi) => msi,
        Err(err) => {
            error!("Failed to read MSI {input_file}.\n{err}");
            return ExitCode::FAILURE;
        }
    };

    let ret = match list_item {
        ATL::Author => list_author(msi),
        ATL::Tables => list_tables(msi),
        ATL::TableColumns { table } => list_table_columns(msi, table),
        ATL::TableContents { table } => list_table_contents(&mut msi, table),
    };
    if let Err(err) = ret {
        error!("Error while trying to inspect MSI.\n{err}");
        return ExitCode::FAILURE;
    } else {
        return ExitCode::SUCCESS;
    }
}

pub(crate) fn validate_paths(input_file: &Utf8PathBuf) -> Result<()> {
    let err_msg = if !input_file.exists() {
        Some(format!("Input file {} does not exist", input_file))
    } else if !input_file.is_file() {
        Some(format!("Input file {} is not a file", input_file))
    } else {
        None
    };

    err_msg.with_context(|| format!("Failed to validate paths."))?;
    Ok(())
}

fn list_author(msi: Package<File>) -> Result<String> {
    debug!("Listing author of MSI");
    let author = msi
        .summary_info()
        .author()
        .with_context(|| format!("Couldn't find author in MSI"))?;
    Ok(author.to_owned())
}

fn list_tables(msi: Package<File>) -> Result<String> {
    debug!("Listing tables in MSI");
    let tables = msi.tables().map(|t| t.name()).collect::<Vec<&str>>();
    Ok(tables.join("\n"))
}

/// List the columns present in the given table
fn list_table_columns(msi: Package<File>, table: SharedStr) -> Result<String> {
    debug!("Listing the columns of table {} in MSI", table);
    let table = match msi.get_table(&table) {
        Some(table) => table,
        None => {
            bail!("Table {} could not be found in MSI", table)
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
fn list_table_contents(
    msi: &mut Package<File>,
    table_name: SharedStr,
) -> Result<String> {
    debug!("Listing the contents of table {} in MSI", table_name);

    let rows = msi
        .select_rows(Select::table(table_name.to_string()))
        .with_context(|| {
            format!("Failed to get rows from table {table_name}")
        })?;

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
        .with_context(|| format!("Failed to display table {table_name}"))?
        .to_string())
}
