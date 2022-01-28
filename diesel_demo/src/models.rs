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

#[derive(Queryable)]
pub struct Channel {
    pub id: i32,
    pub title: String,
    pub name: String,
    pub description: String,
    pub url: String,
}

#[derive(Insertable)]
#[table_name="channels"]
pub struct NewChannel {
    pub title: String,
    pub name: String,
    pub description: String,
    pub url: String,
}