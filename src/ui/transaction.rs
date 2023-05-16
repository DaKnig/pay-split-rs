use adw::glib;
use glib::GString;

pub struct Transaction {
    pub from: GString,
    pub to: GString,
    pub amount: f32,
}

// now the Transaction widget

// mod imp;

// use glib::Object;
// use gtk::{gio, glib, Application};

// glib::wrapper! {
//     pub struct TransactionWidget(ObjectSubclass<imp::Box>)
//         @extends gtk::Box, gtk::Widget,
//         @implements gtk::Accessible, gtk::Buildable, gtk::Orientable,
//                     gtk::ConstraintTarget;
// }

// impl TransactionWidget {
//     pub fn new(trans: &Transaction) -> Self {
//         // Create new TransactionWidget
//         Object::builder()
//             .property("from", trans.from)
//             .property("to", trans.to)
//             .property("amount", trans.amount)
//             .build()
//     }
// }
