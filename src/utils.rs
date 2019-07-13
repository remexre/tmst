use crate::{models::Time, schema::times::dsl::*};
use chrono::{offset::TimeZone, DateTime, Datelike, Local, NaiveDate, Utc};
use diesel::prelude::*;
use std::ops::Range;

pub fn current_id(conn: &SqliteConnection) -> QueryResult<Option<i32>> {
    run_opt(times.filter(end.is_null()).select(id), conn)
}

pub fn current_project(conn: &SqliteConnection) -> QueryResult<Option<String>> {
    run_opt(times.filter(end.is_null()).select(project), conn)
}

pub fn current_time(conn: &SqliteConnection) -> QueryResult<Option<Time>> {
    run_opt(times.filter(end.is_null()), conn)
}

pub fn day_bounds(date: NaiveDate) -> Range<DateTime<Utc>> {
    let end_date = NaiveDate::from_num_days_from_ce(date.num_days_from_ce() + 1);
    let start_ndt = date.and_hms(0, 0, 0);
    let end_ndt = end_date.and_hms(0, 0, 0);

    let start_dt = Local.from_local_datetime(&start_ndt).unwrap();
    let end_dt = Local.from_local_datetime(&end_ndt).unwrap();

    start_dt.with_timezone(&Utc)..end_dt.with_timezone(&Utc)
}

pub fn expand_time_format(string: &mut String, time: &Time) {
    assert!(time.end.is_none());

    while let Some(i) = string.rfind("%p") {
        string.replace_range(i..i + 2, &time.project);
    }

    let elapsed = Utc::now() - DateTime::from_utc(time.start, Utc);
    let hrs = (elapsed.num_hours() as f32) + (elapsed.num_minutes() as f32 / 60.);
    let hrs_str = format!("{:.02}", hrs);

    while let Some(i) = string.rfind("%h") {
        string.replace_range(i..i + 2, &hrs_str);
    }
}

fn run_opt<Conn, Query, T>(query: Query, conn: &Conn) -> QueryResult<Option<T>>
where
    Conn: Connection,
    Query: diesel::query_dsl::LoadQuery<Conn, T> + RunQueryDsl<Conn>,
{
    match query.get_result(&conn) {
        Ok(x) => Ok(Some(x)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(err) => Err(err),
    }
}
