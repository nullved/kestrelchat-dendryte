use crate::api::guards::auth_context::AuthContext;
use crate::api::guards::rate_limit::WithinRateLimit;
use crate::database::postgres::{
  connection::Database, operations::guilds::get_guild as postgres_get_guild,
  operations::guilds::leave_guild as postgres_leave_guild,
};
use crate::errors::AppError;
use rocket::State;
use rocket::http::Status;
use rocket_okapi::openapi;

#[openapi(tag = "Guilds")]
#[delete("/<guild_id>/leave")]
pub async fn remove_guild(
  _within_rate_limit: WithinRateLimit,
  postgres: &State<Database>,
  auth_ctx: AuthContext,
  guild_id: &str,
) -> Result<Status, AppError> {
  let user_id = auth_ctx.user_id;

  let guild = postgres_get_guild(postgres, guild_id, &user_id)
    .await
    .map_err(|_| AppError::not_found("GUILD_NOT_FOUND"))?;

  if guild.owner_id == user_id {
    return Err(AppError::forbidden("GUILD_OWNER"));
  }

  postgres_leave_guild(postgres, guild_id, &user_id)
    .await
    .map_err(AppError::from)?;

  Ok(Status::NoContent)
}
