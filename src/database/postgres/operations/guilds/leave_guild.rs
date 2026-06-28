use crate::database::postgres::connection::Database;
use crate::database::postgres::error::DatabaseError;

pub async fn leave_guild(
  postgres: &Database,
  guild_id: &str,
  user_id: &str,
) -> Result<(), DatabaseError> {
  let rows = sqlx::query(
    r#"
        DELETE FROM guild_members
        WHERE guild_id = $1, user_id = $2
        "#,
  )
  .bind(&guild_id)
  .bind(user_id)
  .execute(postgres.pool())
  .await
  .map_err(DatabaseError::from_sqlx)?
  .rows_affected();

  if rows == 0 {
    return Err(DatabaseError::NotFound);
  }

  Ok(())
}
