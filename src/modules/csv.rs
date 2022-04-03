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
    akasha_blue_boxes: String,
    akasha_no_blue_boxes: String,
    akasha_trash: String,
    akasha_coronation_rings: String,
    akasha_lineage_rings: String,
    akasha_intricacy_rings: String,
    akasha_gold_bars: String,
    pbhl_blue_boxes: String,
    pbhl_no_blue_boxes: String,
    pbhl_coronation_rings: String,
    pbhl_lineage_rings: String,
    pbhl_intricacy_rings: String,
    pbhl_gold_bars: String,
    gohl_blue_boxes: String,
    gohl_no_blue_boxes: String,
    gohl_trash: String,
    gohl_coronation_rings: String,
    gohl_lineage_rings: String,
    gohl_intricacy_rings: String,
    gohl_gold_bars: String,
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
            akasha_blue_boxes: get_db_value("akasha_blue_boxes"),
            akasha_no_blue_boxes: get_db_value("akasha_no_blue_boxes"),
            akasha_trash: get_db_value("akasha_trash"),
            akasha_coronation_rings: get_db_value("akasha_coronation_rings"),
            akasha_lineage_rings: get_db_value("akasha_lineage_rings"),
            akasha_intricacy_rings: get_db_value("akasha_intricacy_rings"),
            akasha_gold_bars: get_db_value("akasha_gold_bars"),
            pbhl_blue_boxes: get_db_value("pbhl_blue_boxes"),
            pbhl_no_blue_boxes: get_db_value("pbhl_no_blue_boxes"),
            pbhl_coronation_rings: get_db_value("pbhl_coronation_rings"),
            pbhl_lineage_rings: get_db_value("pbhl_lineage_rings"),
            pbhl_intricacy_rings: get_db_value("pbhl_intricacy_rings"),
            pbhl_gold_bars: get_db_value("pbhl_gold_bars"),
            gohl_blue_boxes: get_db_value("gohl_blue_boxes"),
            gohl_no_blue_boxes: get_db_value("gohl_no_blue_boxes"),
            gohl_trash: get_db_value("gohl_trash"),
            gohl_coronation_rings: get_db_value("gohl_coronation_rings"),
            gohl_lineage_rings: get_db_value("gohl_lineage_rings"),
            gohl_intricacy_rings: get_db_value("gohl_intricacy_rings"),
            gohl_gold_bars: get_db_value("gohl_gold_bars"),
        };

        wtr.serialize(logdata)?;
        let no_reset = get_settings_value("no_reset");
        if no_reset == "1" {
            crate::modules::database::reset_log();
        }
        wtr.flush()?;
    }
    Ok(())
}
