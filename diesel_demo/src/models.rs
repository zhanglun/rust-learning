use super::schema::posts;
use super::schema::channels;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Insertable, Queryable)]
#[table_name="channels"]
pub struct Channel<'a> {
    pub title: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub url: &'a str,
}