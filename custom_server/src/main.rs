mod cors;
mod db;
mod error;
mod utils;

pub mod prelude {
    pub use crate::cors::*;
    pub use crate::db::*;
    pub use crate::error::*;
    pub use crate::utils::*;
    pub use crate::W;
}

pub struct W<T>(pub T);

#[macro_use]
extern crate rocket;
use rocket::{serde::json::Json, State};

use prelude::*;

#[post("/app_name/<app_name>/owner_id/<owner_id>")]
async fn init_app(app_name: &str, owner_id: &str, db: &State<KursDB>) -> String {
    let app = db.inner().get_app(app_name).await;
    let res = match app {
        Ok(app) => {
            if app.owner_id.as_str() == owner_id {
                "Success".to_string()
            } else {
                "InvalidOwnerId".to_string()
            }
        }
        Err(err) => err.to_string(),
    };
    res
}

#[get("/user/login/<login>/password/<password>")]
async fn get_user(login: &str, password: &str, db: &State<KursDB>) -> String {
    let user = db.inner().get_user(login).await;
    match user {
        Ok(user) => "Success".to_string(),
        Err(_) => "Authorized user does not exist".to_string(),
    }
}

#[post("/user/app/<app>/login/<login>/password/<password>/host_name/<host_name>")]
async fn add_user(
    login: &str,
    password: &str,
    app: &str,
    host_name: &str,
    db: &State<KursDB>,
) -> Json<User> {
    db.inner().add_user(login, password, app).await.unwrap();
    db.set_user_hostname(login, host_name).await.unwrap();
    let user = db.get_user(login).await.unwrap();
    Json(user)
}

#[get("/user/login/<login>/host_name/<host_name>")]
async fn get_user_host_name(login: &str, host_name: &str, db: &State<KursDB>) -> String {
    let host_name = db.inner().get_user_host_name(login).await.unwrap();
    host_name
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let db = KursDB::new().await?;

    db.add_app("GMOD", "1234").await?;

    let _rocket = rocket::build()
        .mount(
            "/",
            routes![get_user, add_user, get_user_host_name, init_app],
        )
        .attach(CORS)
        .manage(db)
        .launch()
        .await?;
    Ok(())
}
