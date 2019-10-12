use serde::*;

#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub name: String,
  pub display_name: String,
}
