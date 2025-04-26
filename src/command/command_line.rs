use camino::Utf8PathBuf;
use clap::{arg, Parser, Subcommand};
use flexstr::SharedStr;

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
        config: Utf8PathBuf,
        /// Directory storing files used to be added to MSI
        #[arg(short, long)]
        input_directory: Utf8PathBuf,
        /// File path to output. This should end with `.msi`.
        #[arg(short, long)]
        output_path: Utf8PathBuf,
    },
    Inspect {
        /// Path to MSI to read from
        #[arg(short, long)]
        input_file: Utf8PathBuf,

        #[command(subcommand)]
        list_args: AllowedToList,
    },
}

#[derive(Subcommand)]
#[group(required = true, multiple = false)]
pub(crate) enum AllowedToList {
    // List the author of the MSI
    Author,
    // List tables present in the MSI
    Tables,
    // List the columns that a given table has.
    TableColumns { table: SharedStr },
    // List the contents of a given table
    TableContents { table: SharedStr },
}
