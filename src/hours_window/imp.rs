use super::rows::Rows;

use crate::row_headers::RowHeaders;

use adw::subclass::prelude::*;

use glib::subclass::InitializingObject;

use gtk::traits::WidgetExt;
use gtk::{gio, glib};

use std::cell::RefCell;

#[derive(gtk::CompositeTemplate, Default)]
#[template(resource = "/org/synthesis_power/invoice_generator/hours_window.ui")]
pub struct HoursWindow {
    #[template_child]
    pub hours_window_subtitle_label: TemplateChild<gtk::Label>,

    #[template_child]
    pub back_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub save_hours_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub grid: TemplateChild<gtk::Grid>,

    pub row_headers: RefCell<Option<RowHeaders>>,
    pub rows: RefCell<Option<Rows>>,
    pub hours_file: RefCell<Option<gio::File>>,
}

#[glib::object_subclass]
impl ObjectSubclass for HoursWindow {
    const NAME: &'static str = "HoursWindow";
    type Type = super::HoursWindow;
    type ParentType = adw::Window;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
        class.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for HoursWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.init_labels();
        obj.setup_row_headers();
        obj.setup_rows();
        obj.setup_callbacks();
    }
}

#[gtk::template_callbacks]
impl HoursWindow {
    #[template_callback]
    fn handle_required_entries(&self) {
        let save_hours_button: gtk::Button = self.obj().save_hours_button();
        let required_entries_full: bool = self.obj().has_required_fields();
        let types_correct: bool = self.obj().has_correct_types();

        if required_entries_full && types_correct {
            save_hours_button.set_sensitive(true);
        } else {
            save_hours_button.set_sensitive(false);
        }
    }
}

impl WidgetImpl for HoursWindow {}

impl WindowImpl for HoursWindow {}

impl ApplicationWindowImpl for HoursWindow {}

impl AdwApplicationWindowImpl for HoursWindow {}

impl AdwWindowImpl for HoursWindow {}
