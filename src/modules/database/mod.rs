use ini::Ini;
use std::path::Path;

use crate::modules::*;

pub fn get_db() -> ini::Ini {
    return Ini::load_from_file(Path::new("./db/log.ini")).unwrap();
}

pub fn save_db(conf: ini::Ini) {
    conf.write_to_file("./db/log.ini").unwrap();
}

pub fn get_settings_value(field: &str) -> String {
    let mut conf = get_db();
    let val = conf
        .with_section(Some("settings"))
        .get(field)
        .unwrap()
        .to_string();
    val
}

pub fn get_db_value(field: &str) -> String {
    let mut conf = get_db();
    let val = conf
        .with_section(Some("drops"))
        .get(field)
        .unwrap()
        .to_string();
    val
}

pub fn set_db_value(field: &str, total: &str) {
    let mut conf = get_db();
    conf.with_section(Some("drops")).set(field, total);
    save_db(conf);
}

pub fn reset_log() {
    let mut conf = get_db();
    conf.with_section(Some("drops"))
        .set("no_blue_boxes", "0")
        .set("blue_boxes", "0")
        .set("coronation_rings", "0")
        .set("lineage_rings", "0")
        .set("intricacy_rings", "0")
        .set("gold_bars", "0");
    save_db(conf);
}

pub fn create_new_log() {
    if let Err(e) = create_path("./db/") {
        println!("Could not create folder: {:?}", e)
    }

    if !Path::new("./db/log.ini").exists() {
        let mut conf = Ini::new();
        conf.with_section(None::<String>).set("encoding", "utf-8");
        conf.with_section(Some("settings"))
            .set("only_blues", "1")
            .set("no_reset", "0");
        conf.with_section(Some("drops"))
            .set("no_blue_boxes", "0")
            .set("blue_boxes", "0")
            .set("coronation_rings", "0")
            .set("lineage_rings", "0")
            .set("intricacy_rings", "0")
            .set("gold_bars", "0");
        save_db(conf);
    }
}
