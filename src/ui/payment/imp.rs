use adw::{glib, gtk, prelude::*};
use glib::{clone, BoxedAnyObject, SignalHandlerId};

use glib::subclass::InitializingObject;
use gtk::{subclass::prelude::*, CompositeTemplate, Entry, TemplateChild};

use std::cell::{RefCell, RefMut};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/null/daknig/pay-split-rs/payment.ui")]
pub struct PaymentWidget {
    #[template_child]
    pub(super) from: TemplateChild<Entry>,
    #[template_child]
    pub(super) amount: TemplateChild<Entry>,
    signal_ids: RefCell<Option<[SignalHandlerId; 2]>>,
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

use super::Payment;
impl PaymentWidget {
    /// unbind the widget from the object
    pub(super) fn unbind_boxed_payment(&self) {
        if let Some(signal_ids) = self.signal_ids.take() {
            for id in signal_ids {
                self.from.get().disconnect(id);
            }
        }
        self.from.get().set_text("");
        self.amount.get().set_text("");
    }
    /// bind info from the widget to the boxed Payment object.
    pub(super) fn bind_boxed_payment(
        &self,
        boxed_payment: BoxedAnyObject,
    ) {
        // check and disconnect previously assigned object.

        if let Some(signal_ids) = self.signal_ids.take() {
            for id in signal_ids {
                eprintln!(
                    "signalid {:#?} was still bound while rebinding",
                    id
                );
                self.from.get().disconnect(id);
            }
        }

        // bind the `from` Entry
        let new_signal_id = self.from.get().connect_changed(
            clone!(@strong boxed_payment => move |from| {
                let mut payment: RefMut<Payment> =
                    boxed_payment.borrow_mut();

                payment.from = from.text();
            }),
        );

        // bind the `amount` Entry
        let new_signal_ids = [
            new_signal_id,
            self.amount.get().connect_changed(move |amount| {
                // get the mutable reference inside the box
                let mut payment = boxed_payment.borrow_mut::<Payment>();
                // parse the text into a f32
                let sum: Result<f32, _> =
                    amount.text().parse().or_else(|err| {
                        // empty entry is not an error
                        if amount.text() == "" {
                            Ok(0.)
                        } else {
                            Err(err)
                        }
                    });
                // if the entry contains an error, style it as such
                payment.amount = match sum {
                    Ok(sum) => {
                        amount.remove_css_class("error");
                        sum
                    }
                    Err(err) => {
                        println!("{:#?}", err);
                        amount.add_css_class("error");
                        f32::NAN
                    }
                };
            }),
        ];

        self.signal_ids.borrow_mut().replace(new_signal_ids);
    }
}
