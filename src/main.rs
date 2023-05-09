mod data; // import and expose back end

mod hours_row;
mod hours_window;
mod main_window;
mod row_headers;

use adw::prelude::*;
use gtk::{gio, glib};

use main_window::MainWindow;

// feel free to change – name must reflect in `resources/resources.gresource.xml` prefix
const APP_ID: &'static str = "org.synthesis_power.invoice_generator";

fn main() -> glib::ExitCode {
    // add resource file
    gio::resources_register_include!("invoice_generator.gresource")
        .expect("failed to register resources");

    // create a new application with the `APP_ID` above
    let app: adw::Application = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // run the application
    app.run()
}

// build the project UI
fn build_ui(app: &adw::Application) {
    let window = MainWindow::new(app); // main app window
    window.present(); // display main app window
}

// uncomment to run back end only:
/*
use invoice_generator::{InvoiceEntry, InvoiceTotals};
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let xls_path = PathBuf::from("<path-to-summaries>");
    let hours_path = PathBuf::from("<path-to-hours>");

    let file_path = PathBuf::from("<path-to-write-to>");

    let invoice_data = invoice_generator::invoice_data(xls_path, hours_path)?;

    let invoice_entries: Vec<InvoiceEntry> = invoice_data.0;
    let invoice_totals: InvoiceTotals = invoice_data.1;

    invoice_generator::write_to_xls(&invoice_entries, &invoice_totals, file_path);

    println!(
        "invoice_entries: {:#.2?}, invoice_totals {:#.2?}",
        &invoice_entries, &invoice_totals
    );

    Ok(())
} */
