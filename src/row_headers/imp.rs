use adw::subclass::prelude::*;

use gtk::gio;
use gtk::glib;

use once_cell::sync::OnceCell;

#[derive(Default)]
pub struct RowHeaders {
    pub headers_labels: OnceCell<gio::ListStore>,
}

#[glib::object_subclass]
impl ObjectSubclass for RowHeaders {
    const NAME: &'static str = "RowHeaders";
    type Type = super::RowHeaders;
}

impl ObjectImpl for RowHeaders {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for RowHeaders {}

impl WindowImpl for RowHeaders {}

impl AdwWindowImpl for RowHeaders {}

impl ApplicationWindowImpl for RowHeaders {}

impl AdwApplicationWindowImpl for RowHeaders {}
