use adw::gio;
use adw::{prelude::*, Application};

use gio::resources_register_include;

mod payment;
mod transaction;

mod window;
use window::Window;

pub fn build_ui(app: &Application) {
    resources_register_include!("pay-split.gresource")
        .expect("Failed to register resources.");

    let window = Window::new(app);

    // finally, the GUI is constructed.
    window.present();
}
