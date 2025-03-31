use clap::{arg, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct App {
    #[arg(long)]
    pub(crate) log_level: Option<String>,
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Build {
        /// Path to config to build from
        #[arg(short, long)]
        config: String,
        /// Directory storing files used to be added to MSI
        #[arg(short, long)]
        input_directory: String,
        /// Filepath to output. This should end with .msi
        #[arg(short, long)]
        output_path: String,
    },
    List {
        /// Path to MSI to read from
        #[arg(short, long)]
        input_file: String,

        #[command(flatten)]
        list_args: ListArgs,
    },
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub(crate) struct ListArgs {
    // List tables present in the MSI
    #[arg(long)]
    pub(crate) tables: bool,
    // List the author of the MSI
    #[arg(long)]
    pub(crate) author: bool,
}
