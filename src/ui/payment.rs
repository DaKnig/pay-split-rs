// now the Transaction widget
mod imp;

use adw::{glib, gtk};
use glib::Object;

glib::wrapper! {
    pub struct PaymentWidget(ObjectSubclass<imp::PaymentWidget>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Orientable,
                    gtk::ConstraintTarget;
}

impl PaymentWidget {
    pub fn new() -> Self {
        Object::builder()
            .build()
    }
}
