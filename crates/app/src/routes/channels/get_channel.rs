use kestrel_common::models::channel::ChannelType;
use kestrel_postgres::{
  connection::Database,
  operations::{
    channels::{get_channel as pg_get_channel, get_channel_member_ids},
    guilds::get_guild as pg_get_guild,
  },
};
use rocket::{State, serde::json::Json};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::Serialize;

use crate::utils::{auth_context::AuthContext, errors::AppError};

#[derive(Serialize, JsonSchema)]
pub struct GetChannelResponse {
  pub id: String,
  pub channel_type: ChannelType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub guild_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  pub member_ids: Vec<String>,
}

#[openapi(tag = "Channels")]
#[get("/<channel_id>")]
pub async fn get_channel(
  postgres: &State<Database>,
  auth_ctx: AuthContext,
  channel_id: &str,
) -> Result<Json<GetChannelResponse>, AppError> {
  let user_id = auth_ctx.user_id;

  let channel = pg_get_channel(postgres, channel_id)
    .await
    .map_err(AppError::from)?;

  match channel.channel_type {
    ChannelType::GuildText => {
      let guild_id = channel.guild_id.as_deref().unwrap_or_default();
      let _guild = pg_get_guild(postgres, guild_id, &user_id)
        .await
        .map_err(|_| AppError::not_found("CHANNEL_NOT_FOUND"))?;

      Ok(Json(GetChannelResponse {
        id: channel.id,
        channel_type: channel.channel_type,
        guild_id: channel.guild_id,
        name: channel.name,
        member_ids: vec![],
      }))
    }
    ChannelType::Direct => {
      let member_ids = get_channel_member_ids(postgres, channel_id)
        .await
        .map_err(AppError::from)?;

      if !member_ids.iter().any(|id| id == &user_id) {
        return Err(AppError::not_found("CHANNEL_NOT_FOUND"));
      }

      Ok(Json(GetChannelResponse {
        id: channel.id,
        channel_type: channel.channel_type,
        guild_id: channel.guild_id,
        name: channel.name,
        member_ids,
      }))
    }
  }
}
