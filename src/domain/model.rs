use serde::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
  Admin,
  PowerUser,
  User,
  Unknown,
}

#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub name: String,
  pub display_name: String,
  pub role: Role,
}
