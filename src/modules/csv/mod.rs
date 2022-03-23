extern crate csv;

use chrono::{DateTime, Local};
use serde::Serialize;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;

use crate::modules::database::{get_db_value, get_settings_value};
use crate::modules::utility::create_path;

#[derive(Debug, Serialize)]
pub struct Record {
    date: String,
    blue_boxes: String,
    no_blue_boxes: String,
    coronation_rings: String,
    lineage_rings: String,
    intricacy_rings: String,
    gold_bars: String,
}

pub fn export_csv() -> Result<(), Box<dyn Error>> {
    let export_time: DateTime<Local> = Local::now();
    let export_four_digit_year = export_time.format("%Y").to_string();
    let export_month = export_time.format("%m").to_string();
    let export_day = export_time.format("%d").to_string();
    if !Path::new("./exports/").exists() {
        create_path("./exports/")?;
    }
    let str_path = format!(
        "./exports/{}-{}-{}.csv",
        &export_four_digit_year, &export_month, &export_day
    );

    if Path::new("./exports/").exists() {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&str_path)
            .unwrap();
        let mut wtr = csv::Writer::from_writer(&file);

        let logdata = Record {
            date: export_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            blue_boxes: get_db_value("blue_boxes"),
            no_blue_boxes: get_db_value("no_blue_boxes"),
            coronation_rings: get_db_value("coronation_rings"),
            lineage_rings: get_db_value("lineage_rings"),
            intricacy_rings: get_db_value("intricacy_rings"),
            gold_bars: get_db_value("gold_bars"),
        };

        wtr.serialize(logdata)?;
        let no_reset = get_settings_value("no_reset");
        if no_reset == "0" {
            crate::modules::database::reset_log();
        }
        wtr.flush()?;
    }
    Ok(())
}
