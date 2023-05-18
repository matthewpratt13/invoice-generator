use crate::hours_window::HoursWindow;
use invoice_generator::InvoiceData;

use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::gio;
use gtk::glib::{self, GString};
use gtk::traits::ButtonExt;

use std::ffi::OsString;
use std::path::PathBuf;

mod imp;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends adw::ApplicationWindow, adw::Window, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gio::Cancellable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[derive(PartialEq)]
enum EntryType {
    Records,
    Hours,
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Records => write!(f, "Records"),
            EntryType::Hours => write!(f, "Hours"),
        }
    }
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn set_enter_hours_label(&self, hours_window: &HoursWindow) -> gtk::Inhibit {
        if let Some(p) = hours_window.hours_file() {
            self.imp().hours_file_entry.set_text(
                p.to_str()
                    .expect("unable to set `enter_hours_label` – invalid path"),
            );
        }

        gtk::Inhibit(false)
    }

    fn set_entry_text(&self, entry_type: &EntryType, path: PathBuf) {
        let entry = match entry_type {
            EntryType::Records => &self.imp().records_file_entry,
            EntryType::Hours => &self.imp().hours_file_entry,
        };

        let file_extension: &str = match entry_type {
            EntryType::Records => "xlsx",
            EntryType::Hours => "csv",
        };

        let file_ext_err: String = format!(
            "{} file should be of type {}",
            entry_type,
            file_extension.to_uppercase()
        );
        let path_err: String = format!("{} path does not point to an existing file", entry_type);

        let msg_dialog = adw::MessageDialog::builder()
            .heading("Error")
            .deletable(true)
            .modal(true)
            .destroy_with_parent(true)
            .transient_for(self)
            .build();
        msg_dialog.add_response("close-response", "Close");
        msg_dialog.set_response_enabled("close-response", true);

        if path.is_file() {
            if path.extension() == Some(&OsString::from(file_extension)) {
                entry.set_text(path.to_str().expect("invalid file"));
            } else {
                msg_dialog.set_body(&file_ext_err);
                msg_dialog.present();
            }
        } else {
            msg_dialog.set_body(&path_err);
            msg_dialog.present();
        }
    }

    fn init_labels(&self) {
        let imp = self.imp();

        imp.main_window_subtitle_label
                .set_text("Open an Excel (XLSX) file containing records with hourly inverter yield and export data. Enter peak and off-peak hours in a new window or open a CSV file containing pre-written hours. Neither file should have a title – only its respective headings.");
        imp.records_file_entry
            .set_placeholder_text(Some("No records file opened"));
        imp.hours_file_entry
            .set_placeholder_text(Some("No hours inputted"));
        imp.generate_entries_button.set_sensitive(false);
    }

    fn setup_callbacks(&self) {
        self.imp().open_records_button.connect_clicked(
            glib::clone!(@weak self as window=> move |_| {
                let filter = gtk::FileFilter::new();
                filter.add_suffix("xlsx");

                let cancellable = None::<&gio::Cancellable>;
                let file_dialog = gtk::FileDialog::builder().title("Open Records").default_filter(&filter).modal(true).build();

                file_dialog.open(Some(&window), cancellable, glib::clone!(@strong window=> move |file| {
                    if let Ok(f) = file {
                        let path: PathBuf = f.path().expect("'Open Records' path does not exist");
                        window.set_entry_text(&EntryType::Records, path);
                    }

                    if window.labels_are_valid() {
                        window.imp().generate_entries_button.set_sensitive(true);
                    } else {
                        window.imp().generate_entries_button.set_sensitive(false)
                    }
                }))
            }),
        );

        self.imp().records_file_entry.connect_icon_release(
            glib::clone!(@weak self as window => move |_, _| {
                window.imp().records_file_entry.buffer().set_text("");
                window.imp().generate_entries_button.set_sensitive(false);
            }),
        );

        self.imp().enter_hours_button.connect_clicked(
            glib::clone!(@weak self as main_window => move |_button| {
                let application: Option<gtk::Application> = main_window.application();

                let hours_window = HoursWindow::new();
                hours_window.set_transient_for(Some(&main_window));
                hours_window.set_application(application.as_ref());

                hours_window.connect_close_request(glib::clone!(@strong main_window=> move |window| {
                    main_window.set_enter_hours_label(window);
                    gtk::Inhibit(false)
                }));

                hours_window.present();
            }),
        );

        self.imp().open_hours_button.connect_clicked(
            glib::clone!(@weak self as window=> move |_| {
                let filter = gtk::FileFilter::new();
                filter.add_suffix("csv");

                let cancellable = None::<&gio::Cancellable>;
                let file_dialog = gtk::FileDialog::builder().title("Open Hours").default_filter(&filter).modal(true).build();

                file_dialog.open(Some(&window), cancellable, glib::clone!(@strong  window=> move |file| {
                    if let Ok(f) = file {
                        let path: PathBuf = f.path().expect("'Open Hours' path does not exist");
                        window.set_entry_text(&EntryType::Hours, path);
                    }

                    if window.labels_are_valid() {
                        window.imp().generate_entries_button.set_sensitive(true);
                    } else {
                        window.imp().generate_entries_button.set_sensitive(false)
                    }
                }));

                window.imp().generate_entries_button.set_sensitive(false);
            }),
        );

        self.imp().hours_file_entry.connect_icon_release(
            glib::clone!(@weak self as window => move |_, _| {
                window.imp().hours_file_entry.buffer().set_text("");
                window.imp().generate_entries_button.set_sensitive(false);
            }),
        );

        self.imp().generate_entries_button.connect_clicked(
            glib::clone!(@weak self as window=> move |_| {
                if window.labels_are_valid() {
                    let records: GString = window.imp().records_file_entry.text();
                    let hours: GString = window.imp().hours_file_entry.text();

                    let xls_path = PathBuf::from(records);
                    let hours_path = PathBuf::from(hours);

                    let invoice_data: InvoiceData = invoice_generator::invoice_data(xls_path, hours_path).expect("unable to generate entries – invalid data");

                    let file_dialog = gtk::FileDialog::builder().title("Save Entries").modal(true).build();
                    let cancellable = gio::Cancellable::current();

                    file_dialog.save(Some(&window), cancellable.as_ref(), glib::clone!(@weak window => move |file| {
                        if let Ok(f) = file {
                            let path: PathBuf = f.path().expect("invalid path");
                            invoice_generator::write_to_xls(invoice_data, path);

                            let msg_dialog = adw::MessageDialog::builder().heading("Success").body("Invoice generated").deletable(true).modal(true).destroy_with_parent(true).transient_for(&window).build();
                            msg_dialog.add_response("close-response", "Close");
                            msg_dialog.set_response_enabled("close-response", true);
                            msg_dialog.present();
                        }
                    }));
                }
            }),
        );
    }

    fn labels_are_valid(&self) -> bool {
        let init_records_file_entry_text = String::from("No file opened");
        let init_hours_file_entry_text = String::from("No hours selected");

        let records_file_entry_text: GString = self.imp().records_file_entry.text();
        let hours_file_entry_text: GString = self.imp().hours_file_entry.text();

        let records_file_path = PathBuf::from(&records_file_entry_text);
        let hours_file_path = PathBuf::from(&hours_file_entry_text);

        if !records_file_entry_text.is_empty() && !hours_file_entry_text.is_empty() {
            if (records_file_entry_text != init_records_file_entry_text)
                && (hours_file_entry_text != init_hours_file_entry_text)
            {
                if records_file_path.is_file()
                    && records_file_path.extension() == Some(&OsString::from("xlsx"))
                {
                    if hours_file_path.is_file()
                        && hours_file_path.extension() == Some(&OsString::from("csv"))
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}
