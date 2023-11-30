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

#[get("/user/<login>")]
async fn get_user(login: &str, db: &State<KursDB>) -> Json<User> {
    let user = db.inner().get_user(login).await.unwrap();
    Json(user)
}

#[post("/user/<app>/<login>/<password>/<host_name>")]
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

#[get("/user/<login>/host_name")]
async fn get_user_host_name(login: &str, db: &State<KursDB>) -> String {
    let host_name = db.inner().get_user_host_name(login).await.unwrap();
    host_name
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let db = KursDB::new().await?;

    db.add_app("GMOD").await?;

    let _rocket = rocket::build()
        .mount("/", routes![get_user, add_user, get_user_host_name])
        .attach(CORS)
        .manage(db)
        .launch()
        .await?;
    Ok(())
}
