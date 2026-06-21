use chrono::Utc;
use sqlx::query_as;
use ulid::Ulid;

use crate::{connection::Database, error::DatabaseError};
use kestrel_common::models::{Channel, channel::ChannelType};

pub async fn create_channel(
  db: &Database,
  channel_type: ChannelType,
  guild_id: Option<&str>,
  name: Option<&str>,
  creator_id: &str,
  recipient_ids: &[String],
) -> Result<Channel, DatabaseError> {
  let id = Ulid::new().to_string();
  let now = Utc::now();

  let channel = query_as::<_, Channel>(
    r#"
        INSERT INTO channels (id, channel_type, guild_id, name, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, channel_type, guild_id, name, created_at, updated_at
        "#,
  )
  .bind(&id)
  .bind(channel_type)
  .bind(guild_id)
  .bind(name)
  .bind(now)
  .bind(now)
  .fetch_one(db.pool())
  .await
  .map_err(DatabaseError::from_sqlx)?;

  let mut all_members: Vec<String> = recipient_ids.to_vec();
  all_members.push(creator_id.to_string());
  all_members.sort();
  all_members.dedup();

  for user_id in &all_members {
    sqlx::query(
      r#"
          INSERT INTO channel_members (channel_id, user_id, joined_at)
          VALUES ($1, $2, $3)
          "#,
    )
    .bind(&channel.id)
    .bind(user_id)
    .bind(now)
    .execute(db.pool())
    .await
    .map_err(DatabaseError::from_sqlx)?;
  }

  Ok(channel)
}

pub async fn find_existing_direct_channel(
  db: &Database,
  user_id: &str,
  other_user_id: &str,
) -> Result<Option<Channel>, DatabaseError> {
  let channels: Vec<Channel> = sqlx::query_as(
    r#"
        SELECT c.id, c.channel_type, c.guild_id, c.name, c.created_at, c.updated_at
        FROM channels c
        INNER JOIN channel_members cm1 ON cm1.channel_id = c.id
        INNER JOIN channel_members cm2 ON cm2.channel_id = c.id
        WHERE c.channel_type = 'direct'
          AND cm1.user_id = $1
          AND cm2.user_id = $2
        "#,
  )
  .bind(user_id)
  .bind(other_user_id)
  .fetch_all(db.pool())
  .await
  .map_err(DatabaseError::from_sqlx)?;

  Ok(channels.into_iter().next())
}
