use adw::glib;
use adw::prelude::*;
use adw::Application;

mod ui;
use ui::build_ui;

const APP_ID: &str = "null.daknig.pay_split";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
