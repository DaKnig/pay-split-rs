use adw::{glib, gtk, prelude::*};
use glib::clone;

use glib::subclass::InitializingObject;
use gtk::{subclass::prelude::*, CompositeTemplate, Entry, TemplateChild};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/null/daknig/pay-split-rs/payment.ui")]
pub struct PaymentWidget {
    #[template_child]
    pub(super) from: TemplateChild<Entry>,
    #[template_child]
    pub(super) amount: TemplateChild<Entry>,
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
    pub(super) fn unbind_payment(&self) {
        self.from.get().set_text("");
        self.amount.get().set_text("");
    }
    /// bind info from the widget to the boxed Payment object.
    pub(super) fn bind_payment(&self, payment: Payment) {
        // bind the `from` Entry
        self.from.get().connect_changed(
            clone!(@strong payment => move |from| {
                payment.set_from(from.text());
            }),
        );

        // bind the `amount` Entry
        self.amount.get().connect_changed(move |amount| {
            // parse the text into a f32
            let sum: Result<_, _> = amount.text().parse().or_else(|err| {
                // empty entry is not an error
                if amount.text() == "" {
                    Ok(0.0f32)
                } else {
                    Err(err)
                }
            });
            // if the entry contains an error, style it as such
            payment.set_valid(sum.is_ok());
            payment.set_amount(sum.clone().unwrap_or(0.));
            match sum {
                Ok(_) => {
                    amount.remove_css_class("error");
                }
                Err(err) => {
                    println!("{:#?}", err);
                    amount.add_css_class("error");
                }
            }
        });
    }
}
