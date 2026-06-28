use crate::common::{
  TestClient, bearer_auth, login, register_test_users, run_with_containers,
};
use rocket::http::StatusClass;
use serde_json::{Value, json};
use std::sync::Arc;

mod common;

async fn create_guild(
  client: &Arc<TestClient>,
  owner_auth_token: &str,
) -> String {
  let guild_name = "Testing Guild";

  let body = json!({
      "name": guild_name
  });
  let client = client.clone();
  let res = client
    .post("/guilds")
    .header(bearer_auth(owner_auth_token))
    .json(&body)
    .dispatch()
    .await;
  assert_eq!(res.status().class(), StatusClass::Success);

  let res_body: Value = res.into_json().await.unwrap();
  assert!(!res_body["id"].is_null());
  assert!(!res_body["owner_id"].is_null());
  assert_eq!(res_body["name"], guild_name);

  res_body["id"].as_str().unwrap().to_string()
}

async fn join_guild(
  client: &Arc<TestClient>,
  guild_id: &String,
  member_auth_token: &str,
) {
  let client = client.clone();
  let path = format!("/guilds/{guild_id}/join");
  let res = client
    .post(path.as_str())
    .header(bearer_auth(member_auth_token))
    .dispatch()
    .await;
  assert_eq!(res.status().class(), StatusClass::Success);
}

#[rocket::async_test]
async fn guild_create_join_leave_delete() {
  run_with_containers(async |_, client| {
    let mut users = register_test_users(&client, 2).await.unwrap();

    let owner = users.pop().unwrap();
    let member = users.pop().unwrap();

    let owner_tokens = login(&client, &owner).await;
    let member_tokens = login(&client, &member).await;

    let guild_id = create_guild(&client, &owner_tokens.auth_token).await;

    join_guild(&client, &guild_id, &member_tokens.auth_token).await;
  })
  .await;
}
