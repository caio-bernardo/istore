use super::Error;
use sqlx::{prelude::FromRow, query_as, query_with};

use super::{
    base::{self, BaseModelControler},
    ModelManager,
};

#[derive(Debug, FromRow)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Debug, FromRow)]
pub struct UserForLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, FromRow)]
pub struct UserForCreate {
    username: String,
    password: String,
}

impl UserForCreate {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub struct UserModelControler;

impl BaseModelControler for UserModelControler {
    const TABLE: &'static str = "user";
}

impl UserModelControler {
    pub async fn get_by_id(mm: &ModelManager, id: i64) -> Result<User, Error> {
        base::get::<Self, User>(mm, id).await
    }

    pub async fn get_by_name(
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<UserForLogin>, Error> {
        let db = mm.db();

        let sql = format!("SELECT * FROM {0} WHERE name=($1)", Self::TABLE);
        let user: Option<UserForLogin> = sqlx::query_as(&sql)
            .bind(username)
            .fetch_optional(db)
            .await?;
        Ok(user)
    }

    pub async fn create(mm: &ModelManager, user: UserForCreate) -> Result<i64, Error> {
        let db = mm.db();

        let (id,): (i64,) =
            query_as("INSERT INTO ? (username, password) VALUES (?, ?) RETURNING id")
                .bind(Self::TABLE)
                .bind(user.username)
                .bind(user.password)
                .fetch_one(db)
                .await?;
        Ok(id)
    }
}
