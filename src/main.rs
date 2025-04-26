pub(crate) mod command;
pub(crate) mod modules;

use clap::Parser;
use modules::helpers::log_return::{error, info};
use std::process::ExitCode;
use command::{builder, lister};

use crate::command::command_line::{App, Commands};

fn main() -> ExitCode {
    // Read the passed in arguments
    let args = App::parse();
    let log_level = match args.log_level {
        Some(level) => level,
        None => "INFO".to_string(),
    };
    // Setup the logger
    let Ok(logger) = flexi_logger::Logger::try_with_env_or_str(log_level.clone()) else {
        error!(
            "Couldn't create a logger using env [$RUST_LOG] or input string [{}]",
            log_level
        );
        return ExitCode::FAILURE;
    };
    logger.start() .expect("Couldn't start the logger");

    info!("Running whimsi...");
    match args.command {
        Commands::Build {
            config,
            input_directory,
            output_path,
        } => builder::build(&config, &output_path),
        Commands::Inspect {
            input_file,
            list_args,
        } => lister::list(&input_file, list_args)
    }
}
