use std::{
  fmt::{Display, Formatter},
  str::FromStr,
};

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, prelude::Type};

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Type,
)]
#[sqlx(type_name = "channel_type")]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum ChannelType {
  Direct,
  GuildText,
}

impl Display for ChannelType {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    let s = match self {
      ChannelType::Direct => "direct",
      ChannelType::GuildText => "guild_text",
    };

    write!(f, "{}", s)
  }
}

impl FromStr for ChannelType {
  type Err = serde_json::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    serde_json::from_str(&format!("\"{s}\""))
  }
}

#[derive(Debug, Clone, FromRow)]
pub struct Channel {
  pub id: String,

  #[sqlx(rename = "channel_type")]
  pub channel_type: ChannelType,

  pub guild_id: Option<String>,
  pub name: Option<String>,

  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct ChannelMember {
  pub channel_id: String,
  pub user_id: String,
  pub joined_at: DateTime<Utc>,
}
