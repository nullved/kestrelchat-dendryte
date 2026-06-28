use crate::database::postgres::connection::Database;
use crate::database::postgres::error::DatabaseError;
use crate::models::GuildMember;
use chrono::Utc;
use sqlx::query_as;

pub async fn join_guild(
  postgres: &Database,
  guild_id: &str,
  user_id: &str,
) -> Result<GuildMember, DatabaseError> {
  let now = Utc::now();

  let guild_member = query_as::<_, GuildMember>(
    r#"
        INSERT INTO guild_members (guild_id, user_id, joined_at)
        VALUES ($1, $2, $3)
        RETURNING guild_id, user_id, joined_at
        "#,
  )
  .bind(&guild_id)
  .bind(user_id)
  .bind(now)
  .fetch_one(postgres.pool())
  .await
  .map_err(DatabaseError::from_sqlx)?;

  Ok(guild_member)
}
