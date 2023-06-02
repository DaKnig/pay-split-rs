use std::collections::{BTreeMap, VecDeque};

use adw::prelude::*;
use adw::{gio, glib, gtk};
use adw::{Application, ApplicationWindow};

use gio::{resources_register_include, ListStore};
use glib::{clone, BoxedAnyObject};
use gtk::{Builder, Button, ListView, NoSelection, SignalListItemFactory};

mod payment;
use payment::{Payment, PaymentWidget};

mod transaction;
use transaction::{Transaction, TransactionWidget};

pub fn build_ui(app: &Application) {
    resources_register_include!("pay-split-2.gresource")
        .expect("Failed to register resources.");

    let builder = Builder::from_resource(
        "/null/daknig/pay-split-rs-2/pay-split-2.ui",
    );

    let window: ApplicationWindow = builder
        .object("window")
        .expect("no object of type AdwApplicationWindow named 'window'");
    window.set_application(Some(app));

    // now create the list thingie:
    // view
    let input_view: ListView = builder.object("input-view").unwrap();
    let output_view: ListView = builder.object("output-view").unwrap();
    // model
    let input_list_store = ListStore::new(BoxedAnyObject::static_type());
    let input_selection_model =
        NoSelection::new(Some(input_list_store.clone()));
    input_view.set_model(Some(&input_selection_model));

    let output_list_store = ListStore::new(Transaction::static_type());
    let output_selection_model =
        NoSelection::new(Some(output_list_store.clone()));
    output_view.set_model(Some(&output_selection_model));
    // factory
    let input_factory = SignalListItemFactory::new();
    input_factory.connect_setup(move |_, list_item| {
        let widget = PaymentWidget::new();
        list_item.set_child(Some(&widget));
    });

    input_factory.connect_bind(move |_, list_item| {
        // Get `Payment` from `ListItem`
        let boxed_payment: BoxedAnyObject = list_item
            .item()
            .and_downcast()
            .expect("The item has to be an `BoxedAnyObject`.");

        // Get `PaymentWidget` from `ListItem`
        let widget: PaymentWidget = list_item
            .child()
            .and_downcast()
            .expect("The child has to be a `PaymentWidget`.");

        // Set "widget" to "payment"
        widget.bind_boxed_payment(boxed_payment);
    });

    input_factory.connect_unbind(move |_, list_item| {
        // Get `PaymentWidget` from `ListItem`
        let widget: PaymentWidget = list_item
            .child()
            .and_downcast()
            .expect("The child has to be a `PaymentWidget`.");

        // unbind
        widget.unbind_boxed_payment();
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

    // adding rows...
    let add_button: Button = builder
        .object("add-button")
        .expect("add-button not found in ui file");
    add_button.connect_clicked(
        clone!(@weak input_list_store => move |_| {
            let payment = BoxedAnyObject::new(Payment::default());
            input_list_store.append(&payment);
        }),
    );

    // splitting...
    let split_button: Button = builder
        .object("split-button")
        .expect("split-button not found in ui file");
    split_button.connect_clicked(move |_| {
        let mut paid = BTreeMap::new();
        let mut total: f32 = 0.;
        for payment in &input_list_store {
            let boxed_payment = payment
                .ok() // safely since we wont change the list store
                .and_downcast::<BoxedAnyObject>()
                .expect("The item has to be an `Payment`.");

            let payment = boxed_payment.borrow::<Payment>();

            *paid.entry(payment.from.clone()).or_insert(0.) +=
                payment.amount;
            total += payment.amount;
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

        output_list_store.remove_all();
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
            output_list_store
                .append(&Transaction::new(&front.1, &back.1, amount));
            front.0 += amount;
            back.0 -= amount;

            // prepare for next iteration
            paid.push_front(front);
        }
        // by now we have drained the list
        println!("debug: normalized paid list");
        for debt in &output_list_store {
            let debt: Transaction =
                debt.ok().and_downcast::<Transaction>().unwrap();
            println!("{}", debt);
            // println!(
            //     "{} -> {}$ -> {}",
            //     debt.from(),
            //     debt.amount(),
            //     debt.to()
            // )
        }
    });

    // finally, the GUI is constructed.
    window.show();
}
