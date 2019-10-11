use crate::schema::*;
use serde::*;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub name: String,
  pub display_name: String,
}
