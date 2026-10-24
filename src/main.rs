use anyhow::{Context, Result};
use chrono::{Days, NaiveDate};
use clap::Parser;
use icalendar::{Calendar, Component, Event, EventLike};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Area name (if not specified, prints available areas)
    #[clap(short, long)]
    area: Option<String>,

    /// Input CSV file path
    input: PathBuf,
}

/// Garbage type number -> display name (blank means no collection)
fn gc_type(code: &str) -> Option<&'static str> {
    Some(match code {
        "1" => "燃やせるごみ",
        "2" => "燃やせないごみ",
        "8" => "びん・缶・ペット",
        "9" => "容器プラ",
        "10" => "雑がみ",
        "11" => "枝・葉・草",
        _ => return None,
    })
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut rdr = csv::Reader::from_path(&args.input)
        .with_context(|| format!("Failed to open {}", args.input.display()))?;

    let headers = rdr.headers()?;
    // 0: Date, 1: Weekday, 2+: Area names

    let Some(area) = args.area else {
        println!("Available areas:");
        for a in headers.iter().skip(2) {
            println!("  {a}");
        }
        return Ok(());
    };
    let idx = headers
        .iter()
        .skip(2)
        .position(|h| h == area)
        .with_context(|| format!("Area not found: {area}"))?
        + 2;

    let mut cal = Calendar::new();
    for rec in rdr.records() {
        let rec = rec?;
        let Some(name) = rec.get(idx).and_then(gc_type) else {
            continue;
        };
        let date_str = rec.get(0).context("Missing date column")?;
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .with_context(|| format!("Invalid date: {date_str}"))?;
        let event = Event::new()
            .uid(&format!("{date}-{area}-{name}@sapporo-gomi-ical"))
            .summary(name)
            .starts(date)
            .ends(date + Days::new(1))
            .done();
        cal.push(event);
    }

    println!("{}", cal.done());
    Ok(())
}
