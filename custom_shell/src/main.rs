use api::CustomApi;

mod api;

fn main() {
    let mut api = CustomApi::new("GMOD", "1234", "http://127.0.0.1:8080");

    api.init().unwrap();
}
