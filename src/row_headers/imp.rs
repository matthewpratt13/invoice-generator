use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::gio;
use gtk::glib::{self, ParamSpec, ParamSpecObject, Value};

use once_cell::sync::{Lazy, OnceCell};

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

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecObject::builder::<gio::ListStore>("headers").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "headers" => {
                let input_value: gio::ListStore = value
                    .get()
                    .expect("The value needs to be of type `gio::ListStore`");
                self.headers_labels
                    .set(input_value)
                    .expect("Could not set header label");
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "headers" => self
                .headers_labels
                .get()
                .expect("Could not get header label")
                .to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for RowHeaders {}

impl WindowImpl for RowHeaders {}

impl AdwWindowImpl for RowHeaders {}

impl ApplicationWindowImpl for RowHeaders {}

impl AdwApplicationWindowImpl for RowHeaders {}
