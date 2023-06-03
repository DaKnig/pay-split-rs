use std::cell::Cell;
use std::cell::RefCell;

use adw::{glib, gtk, subclass::prelude::*};
use glib::{prelude::*, GString, Object, ParamSpec, Properties, Value};

mod transaction_imp {
    // Object holding the state
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::Transaction)]
    pub struct Transaction {
        #[property(get, set)]
        pub from: RefCell<GString>,
        #[property(get, set)]
        pub to: RefCell<GString>,
        #[property(get, set)]
        pub amount: Cell<f32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for Transaction {
        const NAME: &'static str = "TransactionData";
        type Type = super::Transaction;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for Transaction {
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
    pub struct Transaction(ObjectSubclass<transaction_imp::Transaction>);
}

impl Transaction {
    pub fn new(from: &str, to: &str, amount: f32) -> Self {
        Object::builder()
            .property("from", from)
            .property("to", to)
            .property("amount", amount)
            .build()
    }
}

use std::fmt::{self, Display, Formatter};
impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} -> {}$ -> {}", self.from(), self.amount(), self.to())
    }
}

// now the Transaction widget

mod imp;

glib::wrapper! {
    pub struct TransactionWidget(ObjectSubclass<imp::TransactionWidget>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Orientable,
                    gtk::ConstraintTarget;
}

impl TransactionWidget {
    pub fn new() -> Self {
        // Create new TransactionWidget
        Object::builder().build()
    }

    pub fn bind_transaction(&self, transaction: Transaction) {
        // since this is gonna be regenerated each time anyway, might as well
        // treat it as write-only and just clear the model, forcing rebind
        self.imp().from.get().set_text(&transaction.from());
        self.imp().to.get().set_text(&transaction.to());
        self.imp()
            .amount
            .get()
            .set_text(&format!("{}", transaction.amount()));
    }

    /// unbind the widget from the object
    pub(super) fn unbind_transaction(&self) {
        self.imp().from.get().set_text("");
        self.imp().amount.get().set_text("");
        self.imp().to.get().set_text("");
    }
}
