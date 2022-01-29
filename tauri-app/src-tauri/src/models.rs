use super::schema::channels;

#[derive(Queryable)]
pub struct Channel {
  pub id: i32,
  pub title: String,
  pub name: String,
  pub description: String,
  pub url: String,
}

#[derive(Insertable)]
#[table_name = "channels"]
pub struct NewChannel {
  pub title: String,
  pub name: String,
  pub description: String,
  pub url: String,
}