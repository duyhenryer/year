use chrono::{Datelike, TimeZone, Utc};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

mod progress_bar;
use progress_bar::generate_progress_bar;

fn main() -> io::Result<()> {
    let this_year = Utc::now().year();
    let start_time_of_this_year = Utc.with_ymd_and_hms(this_year, 1, 1, 0, 0, 0).unwrap().timestamp_millis();
    let end_time_of_this_year = Utc.with_ymd_and_hms(this_year, 12, 31, 23, 59, 59).unwrap().timestamp_millis();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

    let progress_of_this_year = (current_time - start_time_of_this_year) as f64 / (end_time_of_this_year - start_time_of_this_year) as f64;
    let progress_bar_of_this_year = generate_progress_bar(progress_of_this_year);

    let readme = format!(
        "```\n\
        ⏳ Year Progress {} {:.2} %\n\n\
        ⏰ {}\n\n\
        ```",
        progress_bar_of_this_year,
        progress_of_this_year * 100.0,
        Utc::now().to_rfc2822()
    );

    let readme_path = "README.md";

    // Open README.md in write mode, create it if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true) // Truncate to clear old content
        .write(true)
        .open(readme_path)?;

    // Write the new content to README.md
    file.write_all(readme.as_bytes())?;

    println!("{}", readme);

    Ok(())
}