mod cli;
use chrono::{DateTime, Local, TimeZone, Utc};
use chrono_tz::{Tz, TZ_VARIANTS};
use cli::{Cli, Parser};
use eyre::{Context, Result};

fn dtf<T>(dt: DateTime<T>) -> String
where
    T: TimeZone,
{
    dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let dt = match &args.date {
        Some(date) => DateTime::parse_from_rfc3339(date)?,
        None => chrono::offset::Local::now().into(),
    };
    let dt_utc = dt.with_timezone(&Utc);
    let dt_local: DateTime<Local> = DateTime::from(dt_utc);
    let width = if args.all { 32 } else { 6 };

    if !args.quiet && args.date.is_some() {
        println!("{:>width$}: {}", "Input", dtf(dt), width = width);
    }
    if !(args.quiet && args.to.is_some()) {
        if args.quiet {
            println!("{}", dtf(dt_utc));
        } else {
            println!("{:>width$}: {}", "UTC", dtf(dt_utc), width = width);
        }
    }
    if !args.quiet {
        println!("{:>width$}: {}", "Local", dtf(dt_local), width = width);
    }

    if let Some(target) = args.to {
        let tz: Tz = target
            .parse()
            .wrap_err_with(|| format!("error parsing timezone: '{target}'"))?;
        let dt_target = dt.with_timezone(&tz);
        if args.quiet {
            println!("{}", dtf(dt_target));
        } else {
            println!("{:>width$}: {}", "Target", dtf(dt_target), width = width);
        }
    }
    if args.all {
        let mut variants: Vec<Tz> = Vec::from(TZ_VARIANTS);

        if args.order {
            variants.sort_by(|a, b| {
                dt.with_timezone(a)
                    .fixed_offset()
                    .offset()
                    .local_minus_utc()
                    .cmp(
                        &dt.with_timezone(b)
                            .fixed_offset()
                            .offset()
                            .local_minus_utc(),
                    )
            });
        }
        for tz in &variants {
            let dt_tz = dt.with_timezone(tz);

            println!("{:>width$}: {}", tz.to_string(), dtf(dt_tz), width = width);
        }
    }

    Ok(())
}
