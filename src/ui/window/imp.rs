use std::collections::{BTreeMap, VecDeque};
use std::fmt::Write;

use adw::{gio, glib, gtk, prelude::*, subclass::prelude::*, Leaflet};
use gio::ListStore;
use glib::{g_debug, g_warning, subclass::InitializingObject};
use gtk::{Button, CompositeTemplate, ListView};

use gtk::{NoSelection, SignalListItemFactory};

use crate::ui::payment::{Payment, PaymentWidget};

use crate::ui::transaction::{Transaction, TransactionWidget};

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

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        // now wire everything
        // now create the list thingie:
        // view
        // model
        let input_list_store = self.input_list_store.clone();
        let input_selection_model =
            NoSelection::new(Some(input_list_store));
        let input_view = &self.input_view;
        input_view.set_model(Some(&input_selection_model));

        let output_list_store = self.output_list_store.clone();
        let output_selection_model =
            NoSelection::new(Some(output_list_store));
        let output_view = &self.output_view;
        output_view.set_model(Some(&output_selection_model));

        // factory
        let input_factory = SignalListItemFactory::new();
        input_factory.connect_setup(move |_, list_item| {
            let widget = PaymentWidget::new();
            list_item.set_child(Some(&widget));
        });

        input_factory.connect_bind(move |_, list_item| {
            // Get `Payment` from `ListItem`
            let payment: Payment = list_item
                .item()
                .and_downcast()
                .expect("The item has to be an `Payment`.");

            // Get `PaymentWidget` from `ListItem`
            let widget: PaymentWidget = list_item
                .child()
                .and_downcast()
                .expect("The child has to be a `PaymentWidget`.");

            // Set "widget" to "payment"
            widget.bind_payment(payment);
        });

        input_factory.connect_unbind(move |_, list_item| {
            // Get `PaymentWidget` from `ListItem`
            let widget: PaymentWidget = list_item
                .child()
                .and_downcast()
                .expect("The child has to be a `PaymentWidget`.");

            // unbind
            widget.unbind_payment();
        });

        input_view.set_factory(Some(&input_factory));

        let output_factory = SignalListItemFactory::new();

        output_factory.connect_setup(move |_, list_item| {
            list_item.set_child(Some(&TransactionWidget::new()));
        });

        output_factory.connect_unbind(move |_, list_item| {
            let widget: TransactionWidget = list_item
                .child()
                .and_downcast()
                .expect("The child has to be a `TransactionWidget`.");

            // unbind
            widget.unbind_transaction();
        });

        output_factory.connect_bind(move |_, list_item| {
            // Get `Transaction` from `ListItem`
            let transaction: Transaction = list_item
                .item()
                .and_downcast()
                .expect("The item has to be an `Transaction`.");

            // Get `TransactionWidget` from `ListItem`
            let widget: TransactionWidget = list_item
                .child()
                .and_downcast()
                .expect("The child has to be a `TransactionWidget`.");

            // Set "widget" to "transaction"
            widget.bind_transaction(transaction);
        });
        output_view.set_factory(Some(&output_factory));

        self.add_row(); // start with one empty row
    }
}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}

#[gtk::template_callbacks]
impl Window {
    pub fn maybe_add_row(&self) {
        (&self.input_list_store).into_iter().last();
    }
    #[template_callback]
    pub fn add_row(&self) {
        let payment = super::Payment::default();
        self.input_list_store.append(&payment);
    }
    #[template_callback]
    pub fn back_to_payments(&self) {
        self.leaflet.navigate(adw::NavigationDirection::Back);
    }
    #[template_callback]
    pub fn split(&self) {
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
                break;
            }
        }
        if total.is_nan() {
            g_warning!("pay-split-rs", "please correct your inputs");
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

        {
            let mut paid_list: String = "normalized paid list\n".into();

            for payment in &paid {
                write!(paid_list, "{} -> {}$\n", payment.1, payment.0)
                    .unwrap()
            }
            g_debug!("pay-split-rs", "{}", paid_list);
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
        {
            let mut msg: String = "normalized paid list\n".into();
            for debt in &self.output_list_store {
                let debt: Transaction = debt.ok().and_downcast().unwrap();
                write!(msg, "{}\n", debt).unwrap();
            }
            g_debug!("pay-split-rs", "{}", msg);
        }
        // change the active leaflet page to the result page
        if self.leaflet.is_folded() {
            self.leaflet.navigate(adw::NavigationDirection::Forward);
        }
    }
}
