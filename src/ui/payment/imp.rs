use adw::{glib, gtk, prelude::*};
use glib::{clone, BoxedAnyObject};

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

use super::Payment;
impl PaymentWidget {
    pub fn bind_boxed_payment(&self, boxed_payment: BoxedAnyObject) {
        // we need only propogate widget -> data.
        self.from.get().connect_changed(
            clone!(@strong boxed_payment => move |from| {
            let mut payment = boxed_payment.borrow_mut::<Payment>();
            payment.from = from.text();
            println!("payment changed: {:#?}", payment);
                }),
        );

        self.amount.get().connect_changed(move |amount| {
            let mut payment = boxed_payment.borrow_mut::<Payment>();
            let sum = amount.text().parse::<f32>().or_else(|err| {
                if amount.text() == "" {
                    Ok(0.)
                } else {
                    Err(err)
                }
            });
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
            println!("payment changed: {:#?}", payment);
        });
    }
}
