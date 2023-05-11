use adw::prelude::*;
use adw::{gio, glib, gtk};
use adw::{Application, ApplicationWindow, HeaderBar, Leaflet};

use gtk::{
    Builder, Button, ListItem, ListView, NoSelection,
    SignalListItemFactory,
};
use gio::{resources_register_include, ListStore};
use glib::{BindingFlags};

mod payment;
use payment::PaymentWidget;

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

    // bind 'folded' to the visibility of the buttons

    let leaflet: Leaflet = builder.object("leaflet").unwrap();
    let input_header: HeaderBar =
        builder.object("input-header").unwrap();
    let output_header: HeaderBar =
        builder.object("output-header").unwrap();

    leaflet
        .bind_property(
            "folded",
            &input_header,
            "show-end-title-buttons",
        )
        .flags(BindingFlags::DEFAULT)
        .build();

    leaflet
        .bind_property(
            "folded",
            &output_header,
            "show-start-title-buttons",
        )
        .flags(BindingFlags::DEFAULT)
        .build();

    // now create the list thingie:
    // view
    let input_view: ListView = builder.object("input-view").unwrap();
    let _output_view: ListView = builder.object("output-view").unwrap();
    // model
    let input_list_store =
        ListStore::new(PaymentWidget::static_type());
    let input_selection_model =
        NoSelection::new(Some(input_list_store.clone()));
    input_view.set_model(Some(&input_selection_model));

    // let _: &ListStore = input_model
    //     .model()
    //     .unwrap()
    //     .downcast_ref::<ListStore>()
    //     .unwrap();
    // factory
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let widget = PaymentWidget::new();
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&widget));
    });
    input_view.set_factory(Some(&factory));

    // adding rows...
    let add_button: Button = builder.object("add-button").unwrap();
    add_button.connect_clicked(move |_| {
        let widget = PaymentWidget::new();
        input_list_store.append(&widget);
    });

    // finally, the GUI is constructed.
    window.show();
}
