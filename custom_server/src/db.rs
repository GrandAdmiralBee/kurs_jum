use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use surrealdb::{
    sql::{Object, Thing, Value},
    Response,
};

use crate::macros::map;
use crate::W;

static mut USER_ID: u64 = 0;

#[derive(Serialize, Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub users: std::collections::HashMap<String, User>,
}

impl App {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            users: std::collections::HashMap::default(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        unsafe {
            self.users.insert(USER_ID.to_string(), user);
            USER_ID += 1;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    login: String,
    password: String,
    // creation_date: chrono::DateTime<chrono::Utc>,
    // last_login_date: chrono::DateTime<chrono::Utc>,
    id: Thing,
    host_name: String,
    app: String,
}

impl From<User> for Value {
    fn from(val: User) -> Self {
        map![
            "login".into() => val.login.into(),
            "password".into() => val.password.into(),
            // "creation_date".into() => val.creation_date.into(),
            // "last_login_date".into() => val.last_login_date.into(),
            "id".into() => val.id.into(),
            "host_name".into() => val.host_name.into(),
        ]
        .into()
    }
}

#[derive(Serialize, Debug)]
struct Sub {
    level: u32,
    name: String,
}

#[derive(Serialize, Debug)]
struct License {
    id: String,
    active: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub struct KursDB(Surreal<Db>);

impl KursDB {
    pub async fn new() -> anyhow::Result<Self> {
        let db = Surreal::new::<Mem>(()).await?;

        db.use_ns("kusr").await?;
        db.use_db("kusr").await?;

        Ok(Self(db))
    }

    pub async fn execute(&self, query: &str) -> anyhow::Result<Response> {
        let res = self.0.query(query).await?;
        Ok(res)
    }

    pub async fn add_app(&self, app: &str) -> anyhow::Result<()> {
        let sql = format!("CREATE app:{app} SET name = \"{app}\"");
        Ok(())
    }

    pub async fn get_app_users(&self, app: &str) -> anyhow::Result<Vec<String>> {
        let sql = format!("SELECT login FROM user WHERE app = '{app}'");
        let mut response = self.0.query(sql).await?;
        let users: Vec<String> = response.take("login")?;

        Ok(users)
    }

    pub async fn get_user(&self, login: &str) -> anyhow::Result<User> {
        let sql = format!("SELECT * FROM user:{login}");
        let mut response = self.0.query(sql).await?;

        dbg!(&response);

        let user: Option<User> = response.take(0)?;
        Ok(user.unwrap())
    }

    pub async fn add_user(&self, login: &str, password: &str, app: &str) -> anyhow::Result<()> {
        let sql = format!(
            "CREATE user:{login} SET login = '{login}', password = '{password}', app = '{app}'"
        );
        let response = self.0.query(sql).await?;

        let sql = format!("UPDATE app:{app} SET users += ['user:{login}']");
        let response = self.0.query(sql).await?;
        Ok(())
    }

    pub async fn set_user_hostname(&self, login: &str, host_name: &str) -> anyhow::Result<()> {
        let sql = format!("UPDATE user:{login} SET host_name = '{host_name}'");
        let response = self.0.query(sql).await?;

        Ok(())
    }

    pub async fn get_user_host_name(&self, login: &str) -> anyhow::Result<String> {
        let sql = format!("SELECT host_name FROM user:{login}");
        let mut response = self.0.query(sql).await?;
        let host_name: Option<String> = response.take((0, "host_name"))?;
        Ok(host_name.unwrap_or("".to_string()))
    }
}
