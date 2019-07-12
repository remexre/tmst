#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod cmds;
mod models;
mod options;
mod schema;
mod utils;

embed_migrations!("migrations");

use crate::options::{Options, Subcommand};

use diesel::{sqlite::SqliteConnection, Connection};
use libremexre::errors::{log_err, Result};
use structopt::StructOpt;

fn main() {
    let opts = Options::from_args();
    libremexre::init_logger(opts.verbosity, opts.quiet);

    if let Err(err) = run(opts) {
        log_err(&*err);
        std::process::exit(1)
    }
}

fn run(opts: Options) -> Result<()> {
    let db = opts.db()?;
    let db = db.display().to_string();
    let conn = SqliteConnection::establish(&db)?;
    embedded_migrations::run(&conn)?;

    match opts.subcommand {
        Subcommand::Current { format, otherwise } => {
            cmds::current(&conn, format, otherwise).map(|s| println!("{}", s))
        }
        Subcommand::In { project } => cmds::r#in(&conn, project),
        Subcommand::List {
            after,
            before,
            day,
            today,
        } => cmds::list(&conn, after, before, day, today).map(|chunks| {
            let mut total = 0.0;
            for (date, projects) in chunks {
                let date = date.to_string();
                println!("{}", date);
                for _ in 0..date.len() {
                    print!("=");
                }

                println!("\n");

                let max_project_len = projects.keys().map(|p| p.len()).max().unwrap_or(0);
                for (project, time) in projects {
                    for _ in 0..max_project_len - project.len() {
                        print!(" ");
                    }
                    println!("{} - {:.02}hr", project, time);
                    total += time;
                }

                println!();
            }

            println!("Total: {:.02}hr", total);
        }),
        Subcommand::ListProjects => cmds::list_projects(&conn).map(|projects| {
            for project in projects {
                println!("{}", project);
            }
        }),
        Subcommand::Out => cmds::out(&conn),
    }
}
