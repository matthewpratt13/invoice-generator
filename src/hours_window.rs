mod rows;
mod schedule;

use self::rows::Rows;
use self::schedule::Schedule;

use crate::hours_row::HoursRow;
use crate::row_headers::RowHeaders;

use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::gio;
use gtk::glib::{self, GString, PropertySet, Value};

use std::ffi::OsString;
use std::num::ParseIntError;
use std::path::PathBuf;

mod imp;

glib::wrapper! {
    pub struct HoursWindow(ObjectSubclass<imp::HoursWindow>)
        @extends adw::ApplicationWindow, adw::Window, gtk::ApplicationWindow, gtk::Widget, gtk::Window,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gio::Cancellable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl HoursWindow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    fn grid(&self) -> gtk::Grid {
        self.imp().grid.clone()
    }

    fn save_hours_button(&self) -> gtk::Button {
        self.imp().save_hours_button.clone()
    }

    fn row_headers(&self) -> RowHeaders {
        self.imp()
            .row_headers
            .borrow()
            .clone()
            .expect("unable to get `row_headers` (RowHeaders) for `self` (HoursWindow) – header labels (ListStore<Label>) do not exist)")
    }

    fn rows(&self) -> Rows {
        self.imp().rows.borrow().clone().expect(
            "unable to get `rows` (Rows) for `self` (HoursWindow) – rows (HoursRow) do not exist",
        )
    }

    pub fn hours_file(&self) -> Option<PathBuf> {
        if let Some(hf) = self.imp().hours_file.borrow().clone() {
            hf.path()
        } else {
            None::<PathBuf>
        }
    }

    fn set_hours_file(&self, path: PathBuf) {
        let file = gio::File::for_path(path);
        self.imp().hours_file.set(Some(file))
    }

    fn init_labels(&self) {
        let imp = self.imp();

        imp.hours_window_subtitle_label.set_label("Enter period hours for each day as whole numbers 0–23, where 0 is 12 a.m. and 23 is 11 p.m.. Standard hours are automatically calculated as the difference between specified hours. Only off-peak fields are mandatory (use off-peak hours fields for days when there is only one period and set both hours to 0).");
        imp.save_hours_button.set_sensitive(false);
    }

    fn setup_row_headers(&self) {
        let grid: gtk::Grid = self.grid();

        let headers_labels = gio::ListStore::new(gtk::Label::static_type());

        let mut i: usize = 0;

        while i < 9 {
            let col = i as i32;
            let row = 0;

            let child: Value = grid
                .child_at(col, row)
                .expect("no child at that position in `self.grid()` (gtk::Grid)")
                .to_value();

            if let Ok(c) = child.get::<gtk::Label>() {
                headers_labels.append(&c);
            }

            i += 1;
        }

        let row_headers = RowHeaders::new(headers_labels);
        self.imp().row_headers.set(Some(row_headers));
    }

    fn setup_rows(&self) {
        let grid: gtk::Grid = self.grid();
        let mut hours_rows: Vec<HoursRow> = Vec::new();

        let mut x: usize = 1;

        while x < 8 {
            let hours_entries = gio::ListStore::new(gtk::Entry::static_type());

            let mut weekday_label = gtk::Label::new(None);

            let row_i = x as i32;
            let mut y: usize = 0;

            while y < 9 {
                let col_i = y as i32;

                let child: Value = grid
                    .child_at(col_i, row_i)
                    .expect("no child (Widget) at that position in `self.grid()` (Grid)")
                    .to_value();

                if col_i == 0 {
                    if let Ok(c) = child.get::<gtk::Label>() {
                        weekday_label = c;
                    }
                }

                if let Ok(c) = child.get::<gtk::Entry>() {
                    hours_entries.append(&c);
                }

                y += 1;
            }

            let row = HoursRow::new(weekday_label, hours_entries.clone());
            hours_rows.push(row);

            x += 1;
        }

        let rows = Rows::new(hours_rows);
        self.imp().rows.set(Some(rows));
    }

    fn setup_callbacks(&self) {
        self.imp()
            .back_button
            .connect_clicked(glib::clone!(@weak self as window=> move |_| {
            window.close();
            }));

        self.imp().save_hours_button.connect_clicked(
            glib::clone!(@strong self as window=> move |_| {
                let file_dialog = gtk::FileDialog::builder().title("Save Hours").modal(true).build();

                let rows: Rows = window.rows();
                let row_headers: RowHeaders = window.row_headers();
                let schedule: Schedule = rows.to_schedule(row_headers).expect("unable to convert `rows` (Rows) to Schedule type for `self` (HoursWindow)");

                let cancellable = gio::Cancellable::current();

                file_dialog.save(Some(&window), cancellable.as_ref(), glib::clone!(@weak window => move |file| {
                    if let Ok(f) = file {
                        let mut path: PathBuf = f.path().expect("`path` (PathBuf) for 'Save Hours' dialog (FileDialog) does not exist");

                        if path.as_path().extension() != Some(&OsString::from("csv")) {
                            path.set_extension("csv");
                        }

                        window.set_hours_file(path.clone());

                        let mut wtr = csv::Writer::from_path(path).expect("unable to create `wtr` (Writer) from `path` (PathBuf) at `file` (gio::File) from 'Save Hours' dialog (FileDalog) for `self` (HoursWindow)");

                        wtr.serialize(schedule.row_headers).expect("unable to serialize `row_headers`");
                        wtr.serialize(schedule.mon_hours).expect("unable to serialize `mon_hours`");
                        wtr.serialize(schedule.tue_hours).expect("unable to serialize `tue_hours`");
                        wtr.serialize(schedule.wed_hours).expect("unable to serialize `wed_hours`");
                        wtr.serialize(schedule.thu_hours).expect("unable to serialize `thu_hours`");
                        wtr.serialize(schedule.fri_hours).expect("unable to serialize `fri_hours`");
                        wtr.serialize(schedule.sat_hours).expect("unable to serialize `sat_hours`");
                        wtr.serialize(schedule.sun_hours).expect("unable to serialize `sun_hours`");

                        let msg_dialog = adw::MessageDialog::builder().heading("Success").body("Hours saved").deletable(true).modal(true).destroy_with_parent(true).transient_for(&window).build();
                        msg_dialog.add_response("close-response", "Close");
                        msg_dialog.set_response_enabled("close-response", true);
                        msg_dialog.present();

                        msg_dialog.connect_response(Some("close-response"), glib::clone!(@strong window => move |_,_| {
                            window.close();
                        }));
                    }
                }));
            }),
        );
    }

    fn has_required_fields(&self) -> bool {
        let grid: gtk::Grid = self.grid();
        let mut required_entries: Vec<bool> = Vec::new();

        for x in 1..=7 {
            let row_i: i32 = x;

            for y in 3..=4 {
                let col_i: i32 = y;

                let missing_entry_msg: String = format!(
                    "missing child (gtk::Entry) at row {} col {} in `self.grid` (Grid)",
                    row_i, col_i
                );

                let entry: Value = grid
                    .child_at(col_i, row_i)
                    .expect(missing_entry_msg.as_str())
                    .to_value();

                if let Ok(e) = entry.get::<gtk::Entry>() {
                    let entry_text: GString = e.text();

                    if !entry_text.is_empty() {
                        required_entries.push(true);
                    } else {
                        required_entries.push(false);
                    }
                }
            }
        }

        if !required_entries.contains(&false) {
            true
        } else {
            false
        }
    }

    fn has_correct_types(&self) -> bool {
        let grid: gtk::Grid = self.grid();
        let mut correct_types: Vec<bool> = Vec::new();

        for x in 1..=7 {
            let row_i: i32 = x;

            for y in 1..8 {
                let col_i: i32 = y;

                let missing_entry_msg: String = format!(
                    "missing child (Entry) at row {} col {} in `self.grid` (Grid)",
                    row_i, col_i
                );

                let entry: Value = grid
                    .child_at(col_i, row_i)
                    .expect(missing_entry_msg.as_str())
                    .to_value();

                if let Ok(e) = entry.get::<gtk::Entry>() {
                    let entry_text: GString = e.text();
                    let entry_parsed_result: Result<u32, ParseIntError> = entry_text.parse::<u32>();

                    if (!entry_parsed_result.is_ok() && entry_text != "")
                        || (entry_parsed_result.is_ok() && entry_parsed_result.unwrap() > 23)
                    {
                        correct_types.push(false);
                    } else {
                        correct_types.push(true);
                    }
                }
            }
        }

        if !correct_types.contains(&false) {
            true
        } else {
            false
        }
    }
}
