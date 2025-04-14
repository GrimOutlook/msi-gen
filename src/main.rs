mod builder;
mod command_line;
mod config;
mod helpers;
mod lister;
mod scan;
pub(crate) mod tables;
mod models {
    pub(crate) mod directory;
    pub mod error;
    pub(crate) mod file;
    pub mod sequencer;
}
mod traits {
    pub(crate) mod identifier;
}

use std::process::ExitCode;

use clap::Parser;
use command_line::{AllowedToList, App, Commands};
use helpers::{error, info};

fn main() -> ExitCode {
    // Read the passed in arguments
    let args = App::parse();
    let log_level = match args.log_level {
        Some(level) => level,
        None => "INFO".to_string(),
    };
    // Setup the logger
    flexi_logger::Logger::try_with_env_or_str(log_level.clone())
        .unwrap_or_else(|_| {
            panic!(
                "Couldn't create a logger using env [$RUST_LOG] or input string [{}]",
                log_level
            )
        })
        .start()
        .expect("Couldn't start the logger");

    info!("Running msi-gen...");
    let ret = match args.command {
        Commands::Build {
            config,
            input_directory,
            output_path,
        } => builder::build(&config, &input_directory, &output_path),
        Commands::Inspect {
            input_file,
            list_args,
        } => match lister::list(&input_file, list_args) {
            Ok(output) => {
                println!("{}", output);
                Ok(())
            }
            Err(e) => Err(e),
        },
    };

    match ret {
        Ok(_) => (),
        Err(e) => {
            error!("msi-gen operation failed. Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    info!("msi-gen operation succeeded");
    ExitCode::SUCCESS
}
