use super::schema::times;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Time {
    pub id: i32,
    pub project: String,
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "times"]
pub struct NewTime<'a> {
    pub project: &'a str,
    pub start: NaiveDateTime,
}
