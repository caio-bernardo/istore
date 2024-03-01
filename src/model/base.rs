use serde::Deserialize;
use serde::Serialize;
use sqlx::{sqlite::SqliteRow, FromRow};

use crate::context::Ctx;

use super::Error;
use super::ModelManager;

pub trait BaseModelControler {
    const TABLE: &'static str;
}

pub async fn get<MC, E>(mm: &ModelManager, id: i64) -> Result<E, Error>
where
    MC: BaseModelControler,
    E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
{
    let db = mm.db();
    let entity: E = sqlx::query_as("SELECT * FROM ? where id=?")
        .bind(MC::TABLE)
        .bind(id)
        .fetch_one(db)
        .await?;
    Ok(entity)
}

pub async fn create<MC, E>(mm: &ModelManager, _ctx: &Ctx, entity: E) -> Result<i64, Error>
where
    MC: BaseModelControler,
    E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
{
    /*
     *  query insert into ?1 (?1, ?2, ?3 ...) values (?4, ?5, ?6 ...)
     *
     *  bind TABLE
     *
     *  for key in E as hash
     *       push bind "{key},"
     *  for value in E as hash
     *       push bind "{value},"
     */
    todo!("Can't resolve 'E' bind")
    // FIXME: How to bind E without knowing its params and values names?
}

pub async fn delete<MC, E>(mm: &ModelManager, _ctx: &Ctx, id: i64) -> Result<i64, Error>
where
    MC: BaseModelControler,
    E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
{
    let db = mm.db();
    let (id,) = sqlx::query_as::<_, (i64,)>("DELETE * FROM ?1 WHERE id=?2")
        .bind(MC::TABLE)
        .bind(id)
        .fetch_one(db)
        .await?;
    Ok(id)
}

pub async fn update<MC, E>(mm: &ModelManager, _ctx: &Ctx, id: i64, entity: E) -> Result<E, Error>
where
    MC: BaseModelControler,
    E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
{
    todo!();
}
