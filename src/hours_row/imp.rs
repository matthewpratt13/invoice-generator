use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::gio;
use gtk::glib::{self, ParamSpec, ParamSpecObject, Value};

use once_cell::sync::{Lazy, OnceCell};

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

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecObject::builder::<gtk::Label>("weekday").build(),
                ParamSpecObject::builder::<gio::ListStore>("hours").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "weekday" => {
                let input_value: gtk::Label = value
                    .get()
                    .expect("The value needs to be of type `gtk::Label`");
                self.weekday_label
                    .set(input_value)
                    .expect("Could not set weekday label");
            }
            "hours" => {
                let input_value: gio::ListStore = value
                    .get()
                    .expect("The value needs to be of type `gio::ListStore`");
                self.hours_entries
                    .set(input_value)
                    .expect("Could not set hours entries");
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "weekday" => self
                .weekday_label
                .get()
                .expect("Could not get weekday label")
                .to_value(),
            "hours" => self
                .hours_entries
                .get()
                .expect("Could not get hours entries")
                .to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for HoursRow {}

impl WindowImpl for HoursRow {}

impl AdwWindowImpl for HoursRow {}

impl ApplicationWindowImpl for HoursRow {}

impl AdwApplicationWindowImpl for HoursRow {}
