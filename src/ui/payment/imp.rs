use adw::{glib, gtk};
use glib::subclass::InitializingObject;
use gtk::{
    subclass::prelude::*, CompositeTemplate, Entry, TemplateChild,
};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/null/daknig/pay-split-rs-2/payment.ui")]
pub struct PaymentWidget {
    #[template_child]
    pub from: TemplateChild<Entry>,
    #[template_child]
    pub amount: TemplateChild<Entry>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for PaymentWidget {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "Payment";
    type Type = super::PaymentWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PaymentWidget {}
impl WidgetImpl for PaymentWidget {}
impl BoxImpl for PaymentWidget {}
