use std::process::Command;

fn main() {
    let mut keyauthapp = keyauth::v1_2::KeyauthApi::new(
        "App",
        "wHKFW6oBr3",
        "062feeb0e4bb721f92d5c84bafbab4979f8a46d3fe321b745261647c710ced6b",
        "1.0",
        "https://keyauth.win/api/1.2/", // This is the API URL, change this to your custom domain if you have it enabled
    );

    keyauthapp.init(None).unwrap();

    keyauthapp
        .login(
            "Karim".to_string(),
            "12345678".to_string(),
            Some("141DC34C-1F00-11B2-A85C-AB9EDDDA2374".to_string()),
        )
        .unwrap();

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(r"..\app\src-tauri\target\release\app SpecialArg")
            .spawn()
            .unwrap();
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("../app/src-tauri/target/release/app SpecialArg")
            .spawn()
            .unwrap();
    }
}
