use sqlx::query_as;

use crate::{connection::Database, error::DatabaseError};
use kestrel_common::models::Channel;

pub async fn get_channel(
  db: &Database,
  channel_id: &str,
) -> Result<Channel, DatabaseError> {
  let channel = query_as::<_, Channel>(
    r#"
        SELECT id, channel_type, guild_id, name, created_at, updated_at
        FROM channels
        WHERE id = $1
        "#,
  )
  .bind(channel_id)
  .fetch_optional(db.pool())
  .await
  .map_err(DatabaseError::from_sqlx)?;

  channel.ok_or(DatabaseError::NotFound)
}

pub async fn get_channel_member_ids(
  db: &Database,
  channel_id: &str,
) -> Result<Vec<String>, DatabaseError> {
  let members: Vec<String> = sqlx::query_scalar(
    r#"
        SELECT user_id
        FROM channel_members
        WHERE channel_id = $1
        "#,
  )
  .bind(channel_id)
  .fetch_all(db.pool())
  .await
  .map_err(DatabaseError::from_sqlx)?;

  Ok(members)
}
