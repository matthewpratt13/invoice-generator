use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::{gio, glib};

mod imp;

glib::wrapper! {
    pub struct RowHeaders(ObjectSubclass<imp::RowHeaders>)
      @extends adw::Window, gtk::Widget;
}

impl RowHeaders {
    pub fn new(labels: gio::ListStore) -> Self {
        glib::Object::builder().property("headers", labels).build()
    }

    fn headers_labels(&self) -> gio::ListStore {
        self.imp()
            .headers_labels
            .get()
            .expect("unable to get headers labels")
            .clone()
    }

    pub fn to_headers_strings(&self) -> Vec<String> {
        let headers_strings: Vec<String> = self
            .headers_labels()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<gtk::Label>)
            .map(|l| l.label().to_string())
            .collect();

        headers_strings
    }
}
