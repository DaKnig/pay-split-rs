mod imp;

use adw::{glib, gtk, subclass::prelude::*};
use glib::{BoxedAnyObject, GString, Object};

glib::wrapper! {
    pub struct PaymentWidget(ObjectSubclass<imp::PaymentWidget>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Orientable,
                    gtk::ConstraintTarget;
}

impl PaymentWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
    pub fn bind_boxed_payment(&self, boxed_payment: BoxedAnyObject) {
        self.imp().bind_boxed_payment(boxed_payment)
    }
    pub fn unbind_boxed_payment(&self) {
        self.imp().unbind_boxed_payment();
    }
}

impl Default for PaymentWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Payment {
    pub from: GString,
    pub amount: f32,
}

impl Default for Payment {
    fn default() -> Self {
        Self {
            from: "".into(),
            amount: 0.0,
        }
    }
}
