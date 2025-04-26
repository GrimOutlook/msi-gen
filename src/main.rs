pub(crate) mod command;
pub(crate) mod modules;

use clap::Parser;
use std::process::ExitCode;
use command::{builder, lister};

use crate::modules::helpers::log_return::{info, error};
use crate::command::command_line::{App, Commands};

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

    info!("Running msipmbuild...");
    let ret = match args.command {
        Commands::Build {
            config,
            input_directory,
            output_path,
        } => builder::build(&config, &output_path),
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
            error!("msipmbuild operation failed. Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    info!("msipmbuild operation succeeded");
    ExitCode::SUCCESS
}
