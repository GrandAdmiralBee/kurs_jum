use api::CustomApi;

mod api;
mod error;

use std::process::Command;

use sysinfo::{System, SystemExt};

enum AuthOption {
    Reg,
    Auth,
}

fn main() {
    let mut api = CustomApi::new("GMOD", "1234", "http://127.0.0.1:8000");
    api.init().unwrap();

    println!("Для регистрации пользователя, напечатайте Р, для авторизации, напечатайте А");

    let auth_option = loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        if buffer == "R\n" {
            break AuthOption::Reg;
        } else if buffer == "A\n" {
            break AuthOption::Auth;
        } else {
            println!("Данные для ввода неверны");
        }
    };

    match auth_option {
        AuthOption::Reg => {
            let mut system = System::new_all();
            system.refresh_all();

            let host_name = system.host_name().unwrap();

            println!("Введите логин для аккаунта: ");
            let mut login = String::new();
            std::io::stdin().read_line(&mut login).unwrap();
            let login = login.strip_suffix("\n").unwrap();

            println!("Введите пароль для аккаунта: ");
            let mut password = String::new();
            std::io::stdin().read_line(&mut password).unwrap();
            let password = password.strip_suffix("\n").unwrap();

            println!("Аккаунт {} - {} создан", login, password);
            api.authorize_user(&login, &password, &host_name).unwrap();
        }
        AuthOption::Auth => {
            let mut system = System::new_all();
            system.refresh_all();

            let host_name = system.host_name().unwrap();

            println!("Введите логин аккаунта: ");
            let mut login = String::new();
            std::io::stdin().read_line(&mut login).unwrap();
            let login = login.strip_suffix("\n").unwrap();

            println!("Введите пароль аккаунта: ");
            let mut password = String::new();
            std::io::stdin().read_line(&mut password).unwrap();
            let password = password.strip_suffix("\n").unwrap();

            api.login(&login, &password, &host_name).unwrap();
        }
    }

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
