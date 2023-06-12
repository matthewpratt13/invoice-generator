#![windows_subsystem = "windows"]
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

// use std::{error::Error, path::PathBuf};

// fn main() -> Result<(), Box<dyn Error>> {
//     let xls_path = PathBuf::from("<path-to-summaries>");
//     let hours_path = PathBuf::from("res/hours.csv");

//     let file_path = PathBuf::from("<path-to-invoice>");

//     let invoice_data = invoice_generator::invoice_data(xls_path, hours_path)?;

//     invoice_generator::write_to_xls(invoice_data, file_path);

//     Ok(())
// }
