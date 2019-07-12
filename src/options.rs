use chrono::NaiveDate;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Options {
    /// The path to the SQLite database. Defaults to `$XDG_DATA_HOME/tmst.db`.
    #[structopt(long = "db")]
    pub db: Option<PathBuf>,

    /// The subcommand to run.
    #[structopt(subcommand)]
    pub subcommand: Subcommand,

    /// Disables log output.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Increases the verbosity.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,
}

impl Options {
    pub fn db(&self) -> Result<PathBuf, &'static str> {
        self.db
            .clone()
            .or_else(|| {
                dirs::data_dir().map(|mut p| {
                    p.push("tmst.db");
                    p
                })
            })
            .ok_or("Could not determine database location, use --db")
    }
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Gets the current project, if any.
    #[structopt(name = "current")]
    Current {
        /// A format string. `%p` is replaced with the project, `%h` with the current time.
        #[structopt(short = "f", long = "format", default_value = "%p")]
        format: String,

        /// A string to print if no project exists.
        #[structopt(long = "else")]
        otherwise: Option<String>,
    },

    /// Clocks into a project.
    #[structopt(name = "in")]
    In {
        /// The name of the project to clock into.
        project: String,
    },

    /// Creates a listing of times.
    #[structopt(name = "list")]
    List {
        /// Only show times after this date.
        #[structopt(short = "A", long = "after")]
        after: Option<NaiveDate>,

        /// Only show times before this date.
        #[structopt(short = "B", long = "before")]
        before: Option<NaiveDate>,

        /// Only show a single day.
        #[structopt(short = "d", long = "day")]
        day: Option<NaiveDate>,

        /// Only show today.
        #[structopt(short = "t", long = "today")]
        today: bool,
    },

    /// Lists all projects.
    #[structopt(name = "list-projects")]
    ListProjects,

    /// Clocks out of the current project.
    #[structopt(name = "out")]
    Out,
}
