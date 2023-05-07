use adw::subclass::prelude::*;

use gtk::gio;
use gtk::glib;

use once_cell::sync::OnceCell;

#[derive(Default)]
pub struct HoursRow {
    pub weekday_label: OnceCell<gtk::Label>,
    pub hours_entries: OnceCell<gio::ListStore>,
}

#[glib::object_subclass]
impl ObjectSubclass for HoursRow {
    const NAME: &'static str = "HoursRow";
    type Type = super::HoursRow;
}

impl ObjectImpl for HoursRow {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for HoursRow {}

impl WindowImpl for HoursRow {}

impl AdwWindowImpl for HoursRow {}

impl ApplicationWindowImpl for HoursRow {}

impl AdwApplicationWindowImpl for HoursRow {}
