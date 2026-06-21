use kestrel_common::models::channel::ChannelType;
use kestrel_postgres::{
  connection::Database,
  operations::{
    channels::{
      create_channel as pg_create_channel, find_existing_direct_channel,
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
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum CreateChannelRequest {
  GuildText { guild_id: String, name: String },
  Direct { recipients: Vec<String> },
}

#[derive(Serialize, JsonSchema)]
pub struct CreateChannelResponse {
  pub id: String,
  pub channel_type: ChannelType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub guild_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

#[openapi(tag = "Channels")]
#[post("/", data = "<req>")]
pub async fn create_channel(
  postgres: &State<Database>,
  auth_ctx: AuthContext,
  req: Json<CreateChannelRequest>,
) -> Result<Json<CreateChannelResponse>, AppError> {
  let user_id = auth_ctx.user_id;

  match req.into_inner() {
    CreateChannelRequest::GuildText { guild_id, name } => {
      if name.trim().is_empty() {
        return Err(AppError::bad_request("CHANNEL_NAME_EMPTY"));
      }

      let _guild = pg_get_guild(postgres, &guild_id, &user_id)
        .await
        .map_err(|_| AppError::not_found("GUILD_NOT_FOUND"))?;

      let channel = pg_create_channel(
        postgres,
        ChannelType::GuildText,
        Some(&guild_id),
        Some(&name),
        &user_id,
        &[],
      )
      .await
      .map_err(|e| match e {
        kestrel_postgres::error::DatabaseError::CheckViolation(ref c)
          if c == "channel_name_length" =>
        {
          AppError::bad_request("CHANNEL_NAME_INVALID_LENGTH")
        }
        other => AppError::from(other),
      })?;

      Ok(Json(CreateChannelResponse {
        id: channel.id,
        channel_type: channel.channel_type,
        guild_id: channel.guild_id,
        name: channel.name,
      }))
    }

    CreateChannelRequest::Direct { recipients } => {
      if recipients.len() != 1 {
        return Err(AppError::bad_request(
          "DIRECT_CHANNEL_NEEDS_ONE_RECIPIENT",
        ));
      }

      let other_id = &recipients[0];

      if *other_id == user_id {
        return Err(AppError::bad_request("CANNOT_DM_SELF"));
      }

      if let Some(existing) =
        find_existing_direct_channel(postgres, &user_id, other_id)
          .await
          .map_err(AppError::from)?
      {
        return Ok(Json(CreateChannelResponse {
          id: existing.id,
          channel_type: existing.channel_type,
          guild_id: existing.guild_id,
          name: existing.name,
        }));
      }

      let channel = pg_create_channel(
        postgres,
        ChannelType::Direct,
        None,
        None,
        &user_id,
        &[other_id.clone()],
      )
      .await
      .map_err(AppError::from)?;

      Ok(Json(CreateChannelResponse {
        id: channel.id,
        channel_type: channel.channel_type,
        guild_id: channel.guild_id,
        name: channel.name,
      }))
    }
  }
}
