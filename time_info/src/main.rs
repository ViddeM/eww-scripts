use chrono::{Datelike, Local, Timelike};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    hour: bool,
    #[arg(long)]
    minutes: bool,
    #[arg(long)]
    date: bool,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let args_count = vec![args.hour, args.minutes, args.date]
        .into_iter()
        .filter(|&b| b == true)
        .count();

    eyre::ensure!(
        args_count > 0,
        "Neither --hour, --minutes nor --date was specified"
    );

    eyre::ensure!(
        args_count == 1,
        "More than one output type was specified, please specify at most 1."
    );

    let now = Local::now();
    if args.hour {
        print!("{:02}", now.hour());
    }

    if args.minutes {
        print!("{:02}", now.minute());
    }

    if args.date {
        let date = now.date_naive();
        print!(
            "{} {}/{} - {}",
            date.weekday(),
            date.day(),
            date.month(),
            date.year()
        );
    }

    Ok(())
}
