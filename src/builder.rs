use std::{
    fs::{read_to_string, File},
    io::{Cursor, Write},
    rc::Rc,
};

use log::{error, info};
use msi::{Package, PackageType};

use crate::{config::MsiConfig, files};

// Make a shorthand way to refer to the package cursor for brevity.
pub(crate) type Msi = Package<Cursor<Vec<u8>>>;

pub(crate) fn build(config_path: &str, input_directory: &str, output_path: &str) -> Result<(), ()> {
    // Read the config from the passed in path
    let raw_config = match read_to_string(config_path) {
        Ok(c) => c,
        Err(_) => {
            error!("Failed to open config {}", config_path);
            return Err(());
        }
    };
    let config: Rc<MsiConfig> = match toml::from_str(&raw_config) {
        Ok(c) => Rc::new(c),
        Err(_) => {
            error!("Failed to parse config toml {}", config_path);
            return Err(());
        }
    };

    // Create an empty MSI that we can populate.
    let cursor = Cursor::new(Vec::new());
    let mut package = Package::create(PackageType::Installer, cursor).unwrap();

    // Set the author
    set_author(&mut package, config.clone());

    // Add the files from the input directory
    files::add_files(&mut package, config.clone(), input_directory)?;

    write_msi(package, output_path)
}

fn set_author(package: &mut Msi, config: Rc<MsiConfig>) {
    package
        .summary_info_mut()
        .set_author(config.summary_info.author.clone().unwrap_or_default());
}

fn write_msi(package: Msi, output_path: &str) -> Result<(), ()> {
    let cursor = package.into_inner().unwrap();
    let mut file = match File::create(output_path) {
        Ok(file) => file,
        Err(_) => {
            error!("Failed to open output path {} for writing", output_path);
            return Err(());
        }
    };
    match file.write_all(cursor.get_ref()) {
        Ok(_) => {
            info!("Wrote MSI to {}", output_path);
            Ok(())
        }
        Err(_) => {
            error!("Failed to write MSI to location {}", output_path);
            Err(())
        }
    }
}
