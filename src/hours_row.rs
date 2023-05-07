use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp;

glib::wrapper! {
    pub struct HoursRow(ObjectSubclass<imp::HoursRow>)
      @extends adw::Window, gtk::Widget;
}

impl HoursRow {
    pub fn new(label: gtk::Label, entries: gio::ListStore) -> Self {
        glib::Object::builder()
            .property("weekday", label)
            .property("hours", entries)
            .build()
    }

    pub fn hours_entries(&self) -> gio::ListStore {
        self.imp()
            .hours_entries
            .get()
            .expect("unable to get hours entries")
            .clone()
    }

    pub fn weekday_label(&self) -> gtk::Label {
        self.imp()
            .weekday_label
            .get()
            .expect("unable to get hours weekday label")
            .clone()
    }
}
