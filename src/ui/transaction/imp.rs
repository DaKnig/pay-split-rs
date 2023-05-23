use adw::{glib, gtk};
// use glib::;

use glib::subclass::InitializingObject;
use gtk::{subclass::prelude::*, CompositeTemplate, Label, TemplateChild};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/null/daknig/pay-split-rs-2/payment.ui")]
pub struct TransactionWidget {
    #[template_child]
    pub(super) from: TemplateChild<Label>,
    #[template_child]
    pub(super) amount: TemplateChild<Label>,
    #[template_child]
    pub(super) to: TemplateChild<Label>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TransactionWidget {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "Transaction";
    type Type = super::TransactionWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TransactionWidget {}
impl WidgetImpl for TransactionWidget {}
impl BoxImpl for TransactionWidget {}
