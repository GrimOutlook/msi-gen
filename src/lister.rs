use log::{debug, info};

use crate::AllowedToList as ATL;

pub(crate) fn list(input_file: String, list_item: ATL) -> Result<String, ()> {
    info!("Reading MSI {}", input_file);
    let msi = match msi::open_rw(input_file) {
        Ok(msi) => msi,
        Err(_) => return Err(()),
    };

    match list_item {
        ATL::Tables => {
            debug!("Listing tables in MSI");
            let tables = msi
                .tables()
                .map(|t| t.name())
                .collect::<Vec<&str>>()
                .join("\n");
            Ok(tables)
        }
        ATL::Author => {
            debug!("Listing author of MSI");
            let author = msi.summary_info().author().unwrap_or_default();
            Ok(author.to_owned())
        }
    }
}
