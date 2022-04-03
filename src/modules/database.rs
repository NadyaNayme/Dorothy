use ini::Ini;
use std::path::Path;

use crate::modules::utility::create_path;

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
        .unwrap_or_else(|| "0")
        .to_string();
    val
}

pub fn set_settings_value(field: &str, v: &str) {
    let mut conf = get_db();
    conf.with_section(Some("settings")).set(field, v);
    save_db(conf);
}

pub fn get_db_value(field: &str) -> String {
    let mut conf = get_db();
    let val = conf
        .with_section(Some("drops"))
        .get(field)
        .unwrap_or_else(|| "0")
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
        .set("akasha_no_blue_boxes", "0")
        .set("akasha_blue_boxes", "0")
        .set("akasha_coronation_rings", "0")
        .set("akasha_lineage_rings", "0")
        .set("akasha_intricacy_rings", "0")
        .set("akasha_gold_bars", "0")
        .set("akasha_trash", "0")
        .set("pbhl_no_blue_boxes", "0")
        .set("pbhl_blue_boxes", "0")
        .set("pbhl_coronation_rings", "0")
        .set("pbhl_lineage_rings", "0")
        .set("pbhl_intricacy_rings", "0")
        .set("pbhl_gold_bars", "0")
        .set("gohl_no_blue_boxes", "0")
        .set("gohl_blue_boxes", "0")
        .set("gohl_coronation_rings", "0")
        .set("gohl_lineage_rings", "0")
        .set("gohl_intricacy_rings", "0")
        .set("gohl_trash", "0")
        .set("gohl_gold_bars", "0");
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
            .set("last_active_tab", "0")
            .set("always_on_top", "0")
            .set("only_blues", "0")
            .set("reset_on_export", "1");
        conf.with_section(Some("drops"))
            .set("akasha_no_blue_boxes", "0")
            .set("akasha_blue_boxes", "0")
            .set("akasha_coronation_rings", "0")
            .set("akasha_lineage_rings", "0")
            .set("akasha_intricacy_rings", "0")
            .set("akasha_gold_bars", "0")
            .set("akasha_trash", "0")
            .set("pbhl_no_blue_boxes", "0")
            .set("pbhl_blue_boxes", "0")
            .set("pbhl_coronation_rings", "0")
            .set("pbhl_lineage_rings", "0")
            .set("pbhl_intricacy_rings", "0")
            .set("pbhl_gold_bars", "0")
            .set("gohl_no_blue_boxes", "0")
            .set("gohl_blue_boxes", "0")
            .set("gohl_coronation_rings", "0")
            .set("gohl_lineage_rings", "0")
            .set("gohl_intricacy_rings", "0")
            .set("gohl_trash", "0")
            .set("gohl_gold_bars", "0");
        save_db(conf);
    }
}
