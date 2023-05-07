use crate::hours_window::HoursWindow;

use adw::subclass::prelude::*;

use glib::subclass::InitializingObject;

use gtk::glib;
use gtk::CompositeTemplate;

use std::cell::RefCell;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/synthesis_power/invoice_generator/main_window.ui")]
pub struct MainWindow {
    #[template_child]
    pub open_records_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub enter_hours_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub open_hours_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub generate_entries_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub main_window_subtitle_label: TemplateChild<gtk::Label>,

    #[template_child]
    pub records_file_entry: TemplateChild<gtk::Entry>,

    #[template_child]
    pub hours_file_entry: TemplateChild<gtk::Entry>,

    pub hours_window: RefCell<Option<HoursWindow>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.init_labels();
        obj.setup_callbacks();
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {}

impl AdwWindowImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}

impl AdwApplicationWindowImpl for MainWindow {}
