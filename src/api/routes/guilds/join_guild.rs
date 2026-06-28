use crate::api::guards::auth_context::AuthContext;
use crate::api::guards::rate_limit::WithinRateLimit;
use crate::database::postgres::{
  connection::Database, operations::guilds::join_guild as postgres_join_guild,
};
use crate::errors::AppError;
use chrono::{DateTime, Utc};
use rocket::State;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct JoinGuildResponse {
  pub guild_id: String,
  pub user_id: String,
  pub joined_at: DateTime<Utc>,
}

#[openapi(tag = "Guilds")]
#[post("/<guild_id>/join")]
pub async fn join_guild(
  _within_rate_limit: WithinRateLimit,
  postgres: &State<Database>,
  auth_ctx: AuthContext,
  guild_id: &str,
) -> Result<Json<JoinGuildResponse>, AppError> {
  let user_id = auth_ctx.user_id;

  let guild_member = postgres_join_guild(postgres, guild_id, &user_id)
    .await
    .map_err(AppError::from)?;

  Ok(Json(JoinGuildResponse {
    guild_id: guild_member.guild_id,
    user_id: guild_member.user_id,
    joined_at: guild_member.joined_at,
  }))
}
