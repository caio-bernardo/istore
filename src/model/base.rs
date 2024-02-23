use sqlx::{sqlite::SqliteRow, FromRow};

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
