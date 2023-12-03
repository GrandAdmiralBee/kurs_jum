use api::CustomApi;

mod api;
mod error;

use sysinfo::{System, SystemExt};

fn main() {
    let mut api = CustomApi::new("GMOD", "1234", "http://127.0.0.1:8080");

    api.init().unwrap();

    let mut system = System::new_all();
    system.refresh_all();

    let host_name = system.host_name().unwrap();
    api.authorize_user("admin", "admin", &host_name).unwrap();
    api.login("admin", "admin").unwrap();
}
