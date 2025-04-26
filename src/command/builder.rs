use std::{
    fs::{read_to_string, File}, io::{Cursor, Write}, process::ExitCode, rc::Rc
};

use anyhow::Context;
use camino::Utf8PathBuf;
use msi::{Package, PackageType};

use crate::modules::config::msi_config::MsiConfig;
use crate::modules::{
    helpers::{
        error::MsiError,
        log_return::{error, info},
        scn
    },
    tables,
};

// Make a shorthand way to refer to the package cursor for brevity.
pub(crate) type Msi = Package<Cursor<Vec<u8>>>;

pub(crate) fn build(
    config_path: &Utf8PathBuf,
    output_path: &Utf8PathBuf,
) -> ExitCode {
    info!("Building MSI at output path {}", output_path);
    // Validate paths before continuing
    let Ok(_) = validate_paths(config_path, output_path)? else {
        return ExitCode::FAILURE;
    };

    // The toml library seems to only accept strings as input so we read the whole file in here.
    let raw_config = match read_to_string(config_path).with_context(|| format!("Failed to parse config file [{config_path}]")) {
        Ok(c) => c,
        Err(e) => {
            error!(e);
            return ExitCode::FAILURE;
        }
    };

    // Convert the string output into a usable TOML object.
    let config: Rc<MsiConfig> = match toml::from_str(&raw_config).with_context(|| format!("Failed to parse TOML data from config file {config_path}")) {
        Ok(c) => Rc::new(c),
        Err(e) => {
            error!(e);
            return ExitCode::FAILURE;
        }
    };

    // Create an empty MSI that we can populate.
    let cursor = Cursor::new(Vec::new());
    let mut package = Package::create(PackageType::Installer, cursor).unwrap();

    // Set the author
    set_author(&mut package, config.clone());

    // Add the files from the input directory
    let (directories, files) =
        scan::scan_paths(config.clone(), input_directory).with_context(|| "Failed while scanning file system") else {
        error!(e);
        return ExitCode::FAILURE;
    };

    tables::directory::populate_directory_table(&mut package, &directories)?;
    tables::component::populate_component_table(&mut package, &files)?;
    tables::file::populate_file_table(&mut package, &files)?;

    write_msi(package, output_path)
}

fn set_author(package: &mut Msi, config: Rc<MsiConfig>) {
    package
        .summary_info_mut()
        .set_author(config.summary_info.author.clone().unwrap_or_default());
}

fn write_msi(package: Msi, output_path: &Utf8PathBuf) -> Result<(), MsiError> {
    let cursor = package.into_inner().unwrap();
    let mut file = match File::create(output_path) {
        Ok(file) => file,
        Err(e) => {
            let msg = format!(
                "Failed to open output path {} for writing",
                output_path
            );
            return Err(MsiError::nested(msg, e));
        }
    };
    match file.write_all(cursor.get_ref()) {
        Ok(_) => {
            info!("Wrote MSI to {}", output_path);
            Ok(())
        }
        Err(e) => {
            let msg =
                format!("Failed to write MSI to location {}", output_path);
            Err(MsiError::nested(msg, e))
        }
    }
}

pub(crate) fn validate_paths(
    config_path: &Utf8PathBuf,
    output_path: &Utf8PathBuf,
) -> Result<(), MsiError> {
    // Convert the string (representing the path to scan) into an absolute path.
    let full_path = match camino::absolute_utf8(Utf8PathBuf::from(&output_path))
    {
        Ok(full_path) => full_path,
        Err(e) => {
            return Err(MsiError::nested(
                format!(
                "Failed to get full path for the passed in output path [{}]",
                output_path
            ),
                e,
            ))
        }
    };

    // Since parent returns None when you are at the root folder, it's fine to
    // just use the full path if we hit None as this should just end up being
    // `/` or `C:\` which is valid.
    let output_parent_dir = full_path.as_path().parent().unwrap_or(&full_path);

    let err_msg = if !config_path.exists() {
        Some(error!("Config path {} does not exist", config_path))
    } else if !config_path.is_file() {
        Some(error!("Config path {} is not a file", config_path))
    } else if output_path.parent().is_none() {
        Some(error!(
            "Output path {} is not valid a valid filepath.",
            output_path
        ))
    } else if !output_parent_dir.is_dir() {
        Some(error!(
            "Output parent directory {} is not a valid directory.",
            output_parent_dir
        ))
    } else {
        None
    };

    if let Some(msg) = err_msg {
        return Err(MsiError::short(msg));
    }
    Ok(())
}
