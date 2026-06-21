use chrono::Utc;
use sqlx::query_as;

use crate::{connection::Database, error::DatabaseError};
use kestrel_common::models::Channel;

pub async fn update_channel(
  db: &Database,
  channel_id: &str,
  name: Option<&str>,
) -> Result<Channel, DatabaseError> {
  let updated_at = Utc::now();

  let channel = query_as::<_, Channel>(
    r#"
        UPDATE channels
        SET name = $2, updated_at = $3
        WHERE id = $1
        RETURNING id, channel_type, guild_id, name, created_at, updated_at
        "#,
  )
  .bind(channel_id)
  .bind(name)
  .bind(updated_at)
  .fetch_optional(db.pool())
  .await
  .map_err(DatabaseError::from_sqlx)?;

  channel.ok_or(DatabaseError::NotFound)
}
