use crate::{
    models::{NewTime, Time},
    schema,
    schema::times::dsl::*,
    utils::{current_id, current_project, current_time, day_bounds, expand_time_format},
};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use libremexre::{err, errors::Result};
use std::collections::BTreeMap;

pub fn current(
    conn: &SqliteConnection,
    mut format: String,
    otherwise: Option<String>,
) -> Result<String> {
    match current_time(conn)? {
        Some(time) => {
            expand_time_format(&mut format, &time);
            Ok(format)
        }
        None => otherwise.ok_or_else(|| "Not clocked in to any project!".into()),
    }
}

pub fn r#in(conn: &SqliteConnection, prj: String) -> Result<()> {
    if let Some(prj) = current_project(conn)? {
        return Err(err!("Already clocked in to `{}'!", prj));
    }

    let new_time = NewTime {
        project: &prj,
        start: Utc::now().naive_utc(),
    };

    diesel::insert_into(schema::times::table)
        .values(&new_time)
        .execute(conn)?;

    Ok(())
}

pub fn list(
    conn: &SqliteConnection,
    after: Option<NaiveDate>,
    before: Option<NaiveDate>,
    day: Option<NaiveDate>,
    today: bool,
) -> Result<Vec<(NaiveDate, BTreeMap<String, f32>)>> {
    let mut query = times.select(start).into_boxed();
    if let Some(after) = after {
        let range = day_bounds(after);
        query = query.filter(start.ge(range.start.naive_utc()));
    }
    if let Some(before) = before {
        let range = day_bounds(before);
        query = query.filter(start.lt(range.end.naive_utc()));
    }
    if let Some(day) = day {
        let range = day_bounds(day);
        query = query
            .filter(start.ge(range.start.naive_utc()))
            .filter(start.lt(range.end.naive_utc()));
    }
    if today {
        let range = day_bounds(Local::now().date().naive_local());
        query = query
            .filter(start.ge(range.start.naive_utc()))
            .filter(start.lt(range.end.naive_utc()));
    }

    let mut dates = query
        .distinct()
        .load::<NaiveDateTime>(conn)?
        .into_iter()
        .map(|time| {
            DateTime::<Utc>::from_utc(time, Utc)
                .with_timezone(&Local)
                .naive_local()
                .date()
        })
        .collect::<Vec<_>>();
    dates.sort();
    dates.dedup();

    dates
        .into_iter()
        .map(|date| {
            let range = day_bounds(date);
            let projects = times
                .filter(start.ge(range.start.naive_utc()))
                .filter(start.lt(range.end.naive_utc()))
                .filter(end.is_not_null())
                .load(conn)?
                .into_iter()
                .filter_map(|time: Time| {
                    time.end.map(|e| {
                        let elapsed = DateTime::<Utc>::from_utc(e, Utc)
                            - DateTime::<Utc>::from_utc(time.start, Utc);
                        let hrs =
                            (elapsed.num_hours() as f32) + (elapsed.num_minutes() as f32 / 60.);
                        (time.project, hrs)
                    })
                })
                .collect();
            Ok((date, projects))
        })
        .collect()
}

pub fn list_projects(conn: &SqliteConnection) -> Result<Vec<String>> {
    let projects = times.select(project).distinct().load::<String>(conn)?;
    Ok(projects)
}

pub fn out(conn: &SqliteConnection) -> Result<()> {
    let curid = current_id(conn)?.ok_or("Not clocked in to any project!")?;
    diesel::update(times.filter(id.eq(curid)))
        .set(end.eq(Utc::now().naive_utc()))
        .execute(conn)?;
    Ok(())
}
