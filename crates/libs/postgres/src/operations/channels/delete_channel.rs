use sqlx::query;

use crate::{connection::Database, error::DatabaseError};

pub async fn delete_channel(
  db: &Database,
  channel_id: &str,
) -> Result<(), DatabaseError> {
  let rows = query(
    r#"
        DELETE FROM channels
        WHERE id = $1
        "#,
  )
  .bind(channel_id)
  .execute(db.pool())
  .await
  .map_err(DatabaseError::from_sqlx)?
  .rows_affected();

  if rows == 0 {
    return Err(DatabaseError::NotFound);
  }

  Ok(())
}
