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

    // let mut username = String::new();
    // let mut password = String::new();

    // println!("Print username below:");
    // std::io::stdin().read_line(&mut username).unwrap();
    // println!("Print password below:");
    // std::io::stdin().read_line(&mut password).unwrap();

    keyauthapp
        .login(
            "Timur".to_string(),
            "Timur2709".to_string(),
            Some("141DC34C-1F00-11B2-A85C-AB9EDDDA2374".to_string()),
        )
        .unwrap();

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(r"..\app\src-tauri\target\release\app")
            .spawn()
            .unwrap();
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("../app/src-tauri/target/release/app")
            .spawn()
            .unwrap();
    }
}
