use super::schema::times;

#[derive(Queryable)]
pub struct Time {
    pub id: i32,
    pub project: String,
    pub start: i64,
    pub end: Option<i64>,
}

#[derive(Insertable)]
#[table_name = "times"]
pub struct NewTime<'a> {
    pub project: &'a str,
    pub start: i64,
}
