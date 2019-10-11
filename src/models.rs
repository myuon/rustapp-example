#[derive(Queryable, Debug)]
pub struct User {
  pub id: String,
  pub name: String,
  pub display_name: String,
}
