use adw::{gio, glib, gtk, prelude::*, subclass::prelude::*, Application};
use glib::Object;

use gtk::{NoSelection, SignalListItemFactory};

mod imp;
use crate::ui::payment::{Payment, PaymentWidget};

use crate::ui::transaction::{Transaction, TransactionWidget};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow,
                 gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::Native,
                    gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        let this: Self =
            Object::builder().property("application", app).build();

        // now create the list thingie:
        // view
        // model
        let input_list_store = this.imp().input_list_store.clone();
        let input_selection_model =
            NoSelection::new(Some(input_list_store));
        let input_view = &this.imp().input_view;
        input_view.set_model(Some(&input_selection_model));

        let output_list_store = this.imp().output_list_store.clone();
        let output_selection_model =
            NoSelection::new(Some(output_list_store));
        let output_view = &this.imp().output_view;
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

        this
    }
}
