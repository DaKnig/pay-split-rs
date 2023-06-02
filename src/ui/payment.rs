mod imp;

use adw::{glib, gtk, subclass::prelude::*};
use glib::{prelude::*, GString, Object, ParamSpec, Properties, Value};

use std::cell::{Cell, RefCell};

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
    pub fn bind_payment(&self, payment: Payment) {
        self.imp().bind_payment(payment)
    }
    pub fn unbind_payment(&self) {
        self.imp().unbind_payment();
    }
}

impl Default for PaymentWidget {
    fn default() -> Self {
        Self::new()
    }
}

mod payment_imp {
    // Object holding the state
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::Payment)]
    pub struct Payment {
        #[property(get, set)]
        pub from: RefCell<GString>,
        #[property(get, set)]
        pub amount: Cell<f32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for Payment {
        const NAME: &'static str = "PaymentData";
        type Type = super::Payment;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for Payment {
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(
            &self,
            id: usize,
            value: &Value,
            pspec: &ParamSpec,
        ) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }
}

glib::wrapper! {
    pub struct Payment(ObjectSubclass<payment_imp::Payment>);
}

use std::fmt::{self, Display, Formatter};
impl Display for Payment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} : {}$", self.from(), self.amount())
    }
}

impl Default for Payment {
    fn default() -> Self {
        let obj: Self = Object::builder().build();
        obj.set_from("");
        obj.set_amount(0.0);
        obj
    }
}
