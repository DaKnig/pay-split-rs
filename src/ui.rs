use adw::prelude::*;
use adw::{gio, glib, gtk};
use adw::{Application, ApplicationWindow};

use gio::{resources_register_include, ListStore};
use glib::BoxedAnyObject;
use gtk::{
    Builder, Button, ListItem, ListView, NoSelection,
    SignalListItemFactory,
};

mod payment;
use payment::{Payment, PaymentWidget};

pub fn build_ui(app: &Application) {
    resources_register_include!("pay-split-2.gresource")
        .expect("Failed to register resources.");

    let builder = Builder::from_resource(
        "/null/daknig/pay-split-rs-2/pay-split-2.ui",
    );

    let window: ApplicationWindow = builder.object("window").expect(
        "no object of type AdwApplicationWindow named 'window'",
    );
    window.set_application(Some(app));

    // now create the list thingie:
    // view
    let input_view: ListView = builder.object("input-view").unwrap();
    let _output_view: ListView = builder.object("output-view").unwrap();
    // model
    let input_list_store =
        ListStore::new(BoxedAnyObject::static_type());
    let input_selection_model =
        NoSelection::new(Some(input_list_store.clone()));
    input_view.set_model(Some(&input_selection_model));
    // factory
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let widget = PaymentWidget::new();
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&widget));
    });

    factory.connect_bind(move |_, list_item| {
        // Get `Payment` from `ListItem`
        let list_item = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem");
        let boxed_payment: BoxedAnyObject = list_item
            .item()
            .and_downcast::<BoxedAnyObject>()
            .expect("The item has to be an `IntegerObject`.");

        // Get `PaymentWidget` from `ListItem`
        let widget: PaymentWidget = list_item
            .child()
            .and_downcast::<PaymentWidget>()
            .expect("The child has to be a `PaymentWidget`.");

        // Set "widget" to "payment"
        widget.bind_boxed_payment(boxed_payment);
        // widget.set_label(&integer_object.number().to_string());
    });

    input_view.set_factory(Some(&factory));

    // adding rows...
    let add_button: Button = builder
        .object("add-button")
        .expect("add-button not found in ui file");
    add_button.connect_clicked(move |_| {
        let payment = BoxedAnyObject::new(Payment::default());
        input_list_store.append(&payment);
    });

    // finally, the GUI is constructed.
    window.show();
}
