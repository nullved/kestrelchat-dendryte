use kestrel_common::models::channel::ChannelType;
use kestrel_postgres::{
  connection::Database,
  operations::{
    channels::{
      get_channel as pg_get_channel, update_channel as pg_update_channel,
    },
    guilds::get_guild as pg_get_guild,
  },
};
use rocket::{State, serde::json::Json};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::utils::{auth_context::AuthContext, errors::AppError};

#[derive(Deserialize, JsonSchema)]
pub struct UpdateChannelRequest {
  pub name: String,
}

#[derive(Serialize, JsonSchema)]
pub struct UpdateChannelResponse {
  pub id: String,
  pub channel_type: ChannelType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub guild_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

#[openapi(tag = "Channels")]
#[patch("/<channel_id>", data = "<req>")]
pub async fn update_channel(
  postgres: &State<Database>,
  auth_ctx: AuthContext,
  channel_id: &str,
  req: Json<UpdateChannelRequest>,
) -> Result<Json<UpdateChannelResponse>, AppError> {
  let user_id = auth_ctx.user_id;

  if req.name.trim().is_empty() {
    return Err(AppError::bad_request("CHANNEL_NAME_EMPTY"));
  }

  let channel = pg_get_channel(postgres, channel_id)
    .await
    .map_err(|_| AppError::not_found("CHANNEL_NOT_FOUND"))?;

  match channel.channel_type {
    ChannelType::Direct => {
      return Err(AppError::bad_request("CANNOT_UPDATE_DIRECT_CHANNEL"));
    }
    ChannelType::GuildText => {
      let guild_id = channel.guild_id.as_deref().unwrap_or_default();
      let _guild = pg_get_guild(postgres, guild_id, &user_id)
        .await
        .map_err(|_| AppError::not_found("CHANNEL_NOT_FOUND"))?;
    }
  }

  let updated = pg_update_channel(postgres, channel_id, Some(&req.name))
    .await
    .map_err(|e| match e {
      kestrel_postgres::error::DatabaseError::CheckViolation(ref c)
        if c == "channel_name_length" =>
      {
        AppError::bad_request("CHANNEL_NAME_INVALID_LENGTH")
      }
      other => AppError::from(other),
    })?;

  Ok(Json(UpdateChannelResponse {
    id: updated.id,
    channel_type: updated.channel_type,
    guild_id: updated.guild_id,
    name: updated.name,
  }))
}
