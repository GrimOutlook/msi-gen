use log::{debug, info};

use crate::ListArgs;

pub(crate) fn list(input_file: String, list_args: ListArgs) -> Result<(), ()> {
    info!("Reading MSI {}", input_file);
    let msi = match msi::open_rw(input_file) {
        Ok(msi) => msi,
        Err(_) => return Err(()),
    };

    if list_args.tables {
        debug!("Listing tables in MSI");
        msi.tables().for_each(|f| println!("{:?}", f.name()));
        return Ok(());
    }

    if list_args.author {
        debug!("Listing author of MSI");
        println!("{}", msi.summary_info().author().unwrap_or_default());
        return Ok(());
    }

    Ok(())
}
