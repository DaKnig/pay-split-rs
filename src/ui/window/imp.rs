use std::collections::{BTreeMap, VecDeque};

use adw::{gio, glib, gtk, prelude::*, subclass::prelude::*, Leaflet};
use gio::ListStore;
use glib::subclass::InitializingObject;
use gtk::{Button, CompositeTemplate, ListView};

use crate::ui::{payment::*, transaction::*};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/null/daknig/pay-split-rs/window.ui")]
pub struct Window {
    #[template_child(id = "input-view")]
    pub input_view: TemplateChild<ListView>,
    #[template_child(id = "output-view")]
    pub output_view: TemplateChild<ListView>,
    #[template_child(id = "split-button")]
    pub split_button: TemplateChild<Button>,
    #[template_child(id = "leaflet")]
    pub leaflet: TemplateChild<Leaflet>,

    pub input_list_store: ListStore,
    pub output_list_store: ListStore,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "PaySplitWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    pub fn add_row(&self, _: &Button) {
        let payment = super::Payment::default();
        self.input_list_store.append(&payment);
    }
    #[template_callback]
    pub fn back_to_payments(&self, _: &Button) {
	self.leaflet.get().navigate(adw::NavigationDirection::Back);	
    }
    #[template_callback]
    pub fn split(&self, _: &Button) {
	// update the result page
        let mut paid = BTreeMap::new();
        let mut total: f32 = 0.;
        for payment in self.input_list_store.into_iter() {
            let payment: Payment = payment
                .ok() // safely since we wont change the list store
                .and_downcast()
                .expect("The item has to be a `Payment`.");

            *paid.entry(payment.from()).or_insert(0.) += payment.amount();
            total += payment.amount();
            if !payment.valid() {
                total = f32::NAN;
            }
        }
        if total.is_nan() {
            eprintln!("please correct your inputs");
            return;
        }

        if let Some(x) = paid.get("") {
            if x.abs() < EPSILON {
                paid.remove("");
            }
        }
        let avg = total / (paid.len() as f32);
        // using partial_cmp to ensure no NANs and such.
        let mut paid: Vec<_> = paid.iter().collect();
        paid.sort_unstable_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        const EPSILON: f32 = 0.01;
        // normalize towards the average
        let mut paid: VecDeque<_> =
            paid.into_iter().map(|x| (x.1 - avg, x.0)).collect();

        self.output_list_store.remove_all();
        //output_list_store; // Vec::new();

        println!("debug: normalized paid list");
        for payment in &paid {
            println!("{} -> {}$", payment.1, payment.0)
        }

        // tricking rust into giving me the front and the back :)
        while let (Some(mut front), Some(back)) =
            (paid.pop_front(), paid.back_mut())
        {
            // removes all the tiny leftovers
            if -front.0 <= EPSILON {
                continue;
            } else if back.0 < EPSILON {
                paid.pop_back();
                paid.push_front(front);
                continue;
            }

            // amount to transfer
            let amount = back.0.min(front.0.abs());
            // transfer
            self.output_list_store
                .append(&Transaction::new(front.1, back.1, amount));
            front.0 += amount;
            back.0 -= amount;

            // prepare for next iteration
            paid.push_front(front);
        }
        // by now we have drained the list
        println!("debug: normalized paid list");
        for debt in &self.output_list_store {
            let debt: Transaction = debt.ok().and_downcast().unwrap();
            println!("{}", debt);
        }
	// change the active leaflet page to the result page
	self.leaflet.get().navigate(adw::NavigationDirection::Forward);
    }

}
