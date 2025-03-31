mod command_line;

use clap::Parser;
use command_line::{App, Commands, ListArgs};
use log::{error, info};

fn main() {
    // Read the passed in arguments
    let args = App::parse();
    let log_level = match args.log_level {
        Some(level) => level,
        None => "INFO".to_string(),
    };
    // Setup the logger
    flexi_logger::Logger::try_with_env_or_str(log_level)
        .unwrap()
        .start()
        .unwrap();

    info!("Starting MSI Builder...");
    match args.command {
        Commands::Build {
            config,
            input_directory,
        } => todo!(),
        Commands::List {
            input_file,
            list_args,
        } => {
            list_info(input_file, list_args);
        }
    }
}

fn list_info(input_file: String, list_args: ListArgs) -> Result<(), ()> {
    info!("Reading MSI {}", input_file);
    let msi = match msi::open_rw(input_file) {
        Ok(msi) => msi,
        Err(_) => return Err(()),
    };

    if list_args.tables {
        msi.tables().for_each(|f| println!("{:?}", f.name()));
        return Ok(());
    }

    Ok(())
}
