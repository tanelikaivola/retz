pub use clap::Parser;

/// retimezone - convert timestamps to other timezones
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Date time in only proper format (RFC3339)
    pub date: Option<String>,
    /// Convert to timezone
    #[clap(short, long, value_name = "TZ")]
    pub to: Option<String>,
    /// Quiet, return just either target (if defined) or UTC
    #[clap(short, long, action, group = "output")]
    pub quiet: bool,
    /// List all timezones
    #[clap(short, long, action, group = "output")]
    pub all: bool,
    /// Order by offset instead of alphabetical
    #[clap(short, long, action, requires = "all")]
    pub order: bool,
}
