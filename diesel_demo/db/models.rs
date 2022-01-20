use super::schema::channels;
#[derive(Serialize, Queryable)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub description: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "channels"]
pub struct NewPost<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub description: &'a str,
}