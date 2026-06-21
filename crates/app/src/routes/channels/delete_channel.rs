use kestrel_common::models::channel::ChannelType;
use kestrel_postgres::{
  connection::Database,
  operations::{
    channels::{
      delete_channel as pg_delete_channel, get_channel as pg_get_channel,
    },
    guilds::get_guild as pg_get_guild,
  },
};
use rocket::{State, http::Status};
use rocket_okapi::openapi;

use crate::utils::{auth_context::AuthContext, errors::AppError};

#[openapi(tag = "Channels")]
#[delete("/<channel_id>")]
pub async fn delete_channel(
  postgres: &State<Database>,
  auth_ctx: AuthContext,
  channel_id: &str,
) -> Result<Status, AppError> {
  let user_id = auth_ctx.user_id;

  let channel = pg_get_channel(postgres, channel_id)
    .await
    .map_err(|_| AppError::not_found("CHANNEL_NOT_FOUND"))?;

  match channel.channel_type {
    ChannelType::GuildText => {
      let guild_id = channel.guild_id.as_deref().unwrap_or_default();
      let guild = pg_get_guild(postgres, guild_id, &user_id)
        .await
        .map_err(|_| AppError::not_found("CHANNEL_NOT_FOUND"))?;

      if guild.owner_id != user_id {
        return Err(AppError::forbidden("NOT_GUILD_OWNER"));
      }
    }
    ChannelType::Direct => {
      return Err(AppError::bad_request("CANNOT_DELETE_DIRECT_CHANNEL"));
    }
  }

  pg_delete_channel(postgres, channel_id)
    .await
    .map_err(AppError::from)?;

  Ok(Status::NoContent)
}
