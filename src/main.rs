#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
extern crate ini;

use std::fs;
use std::path::Path;
use nwd::NwgUi;
use nwg::NativeUi;
use ini::Ini;

static BLUE_CHEST: &[u8] = include_bytes!("./images/blue_chest.ico");
static NO_BLUE_CHEST: &[u8] = include_bytes!("./images/no_blue_chest.ico");
static C_RING: &[u8] = include_bytes!("./images/coronation_ring.ico");
static L_RING: &[u8] = include_bytes!("./images/lineage_ring.ico");
static I_RING: &[u8] = include_bytes!("./images/intricacy_ring.ico");
static GOLD_BAR: &[u8] = include_bytes!("./images/hihi.ico");

#[derive(Default, NwgUi)]
pub struct BarTracker {
    #[nwg_control(size: (300, 300), position: (500, 500), title: "Dorothy", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [BarTracker::kill_app], OnInit: [BarTracker::init])]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(family: "Meiryo", size: 16)]
    font: nwg::Font,

    #[nwg_resource(family: "Meiryo UI", size: 14)]
    smallfont: nwg::Font,

    #[nwg_control(text: "心臓蛇団のスーパーシークレットプログラム", font: Some(&data.font), size: (320, 32), position: (10, 5))]
    instructions: nwg::Label,

    // OFC OIFC is bugged. Wait new release but for now just use buttons: https://github.com/gabdube/native-windows-gui/issues/221
    //#[nwg_events((handle, OnImageFrameClick): [BarTracker::change_value], OnImageFrameDoubleClick: [BarTracker::subtract_value] )]

    #[nwg_control(size: (32, 32), position: (10, 40), icon: Some(&nwg::Icon::from_bin(BLUE_CHEST)?))]
    blue_box: nwg::ImageFrame,
        #[nwg_control(text: &mut get_db_value("blue_boxes"), font: Some(&data.font), size: (50, 25), position: (48, 47))]
        blue_box_label: nwg::Label,


    #[nwg_control(size: (32, 32), position: (10, 80), icon: Some(&nwg::Icon::from_bin(NO_BLUE_CHEST)?))]
    no_blue_box: nwg::ImageFrame,
        #[nwg_control(text: &mut get_db_value("no_blue_boxes"), font: Some(&data.font), size: (30, 16), position: (48, 87))]
        no_blue_box_label: nwg::Label,

        #[nwg_control(text: "+1", font: Some(&data.font), size: (32, 32), position: (225, 80))]
        #[nwg_events( OnButtonClick: [BarTracker::add_no_blue_boxes], OnButtonDoubleClick: [BarTracker::add_no_blue_boxes])]
        no_blue_box_add_button: nwg::Button,

        #[nwg_control(text: "-1", font: Some(&data.font), size: (32, 32), position: (260, 80))]
        #[nwg_events(OnButtonClick: [BarTracker::subtract_no_blue_boxes], OnButtonDoubleClick: [BarTracker::subtract_no_blue_boxes])]
        no_blue_box_subtract_button: nwg::Button,


    #[nwg_control(size: (32, 32), position: (10, 120), icon: Some(&nwg::Icon::from_bin(C_RING)?))]
    coronation_ring: nwg::ImageFrame,
        #[nwg_control(text: &mut get_db_value("coronation_rings"), font: Some(&data.font), size: (30, 16), position: (48, 120))]
        coronation_ring_label: nwg::Label,

        #[nwg_control(text: "TBD", font: Some(&data.smallfont), size: (50, 16), position: (48, 134))]
        coronation_ring_percentage: nwg::Label,

        #[nwg_control(text: "+1", font: Some(&data.font), size: (32, 32), position: (225, 120))]
        #[nwg_events( OnButtonClick: [BarTracker::add_coronation_rings], OnButtonDoubleClick: [BarTracker::add_coronation_rings])]
        coronation_rings_add_button: nwg::Button,

        #[nwg_control(text: "-1", font: Some(&data.font), size: (32, 32), position: (260, 120))]
        #[nwg_events(OnButtonClick: [BarTracker::subtract_coronation_rings], OnButtonDoubleClick: [BarTracker::subtract_coronation_rings])]
        coronation_rings_subtract_button: nwg::Button,


    #[nwg_control(size: (32, 32), position: (10, 160), icon: Some(&nwg::Icon::from_bin(L_RING)?))]
    lineage_ring: nwg::ImageFrame,
        #[nwg_control(text: &mut get_db_value("lineage_rings"), font: Some(&data.font), size: (30, 16), position: (48, 160))]
        lineage_ring_label: nwg::Label,

        #[nwg_control(text: "TBD", font: Some(&data.smallfont), size: (50, 16), position: (48, 174))]
        lineage_ring_percentage: nwg::Label,

        #[nwg_control(text: "+1", font: Some(&data.font), size: (32, 32), position: (225, 160))]
        #[nwg_events( OnButtonClick: [BarTracker::add_lineage_rings], OnButtonDoubleClick: [BarTracker::add_lineage_rings])]
        lineage_rings_add_button: nwg::Button,

        #[nwg_control(text: "-1", font: Some(&data.font), size: (32, 32), position: (260, 160))]
        #[nwg_events(OnButtonClick: [BarTracker::subtract_lineage_rings], OnButtonDoubleClick: [BarTracker::subtract_lineage_rings])]
        lineage_rings_subtract_button: nwg::Button,


    #[nwg_control(size: (32, 32), position: (10, 200), icon: Some(&nwg::Icon::from_bin(I_RING)?))]
    intricacy_ring: nwg::ImageFrame,
        #[nwg_control(text: &mut get_db_value("intricacy_rings"), font: Some(&data.font), size: (30, 16), position: (48, 200))]
        intricacy_ring_label: nwg::Label,

        #[nwg_control(text: "TBD", font: Some(&data.smallfont), size: (50, 16), position: (48, 214))]
        intricacy_ring_percentage: nwg::Label,

        #[nwg_control(text: "+1", font: Some(&data.font), size: (32, 32), position: (225, 200))]
        #[nwg_events( OnButtonClick: [BarTracker::add_intricacy_rings], OnButtonDoubleClick: [BarTracker::add_intricacy_rings])]
        intricacy_rings_add_button: nwg::Button,

        #[nwg_control(text: "-1", font: Some(&data.font), size: (32, 32), position: (260, 200))]
        #[nwg_events(OnButtonClick: [BarTracker::subtract_intricacy_rings], OnButtonDoubleClick: [BarTracker::subtract_intricacy_rings])]
        intricacy_rings_subtract_button: nwg::Button,


    #[nwg_control(size: (32, 32), position: (10, 240), icon: Some(&nwg::Icon::from_bin(GOLD_BAR)?))]
    gold_bar: nwg::ImageFrame,
        #[nwg_control(text: &mut get_db_value("gold_bars"), font: Some(&data.font), size: (30, 16), position: (48, 240))]
        gold_bar_label: nwg::Label,

        #[nwg_control(text: "TBD", font: Some(&data.smallfont), size: (50, 16), position: (48, 254))]
        gold_bar_percentage: nwg::Label,

        #[nwg_control(text: "+1", font: Some(&data.font), size: (32, 32), position: (225, 240))]
        #[nwg_events( OnButtonClick: [BarTracker::add_gold_bars], OnButtonDoubleClick: [BarTracker::add_gold_bars])]
        gold_bars_add_button: nwg::Button,

        #[nwg_control(text: "-1", font: Some(&data.font), size: (32, 32), position: (260, 240))]
        #[nwg_events(OnButtonClick: [BarTracker::subtract_gold_bars], OnButtonDoubleClick: [BarTracker::subtract_gold_bars])]
        gold_bars_subtract_button: nwg::Button,
}

impl BarTracker {

    fn init(&self) {
        let em = &self.embed;
        self.window.set_icon(em.icon_str("DOROTHY", None).as_ref());
    }

    fn change_value(&self, field: &str, add: bool) {
        let label = match field {
            "no_blue_boxes" => &self.no_blue_box_label,
            "coronation_rings" => &self.coronation_ring_label,
            "lineage_rings" => &self.lineage_ring_label,
            "intricacy_rings" => &self.intricacy_ring_label,
            "gold_bars" => &self.gold_bar_label,
            _ => &self.blue_box_label
        };

        let mut total_chests = label.text().parse::<i32>().unwrap();

        if label != &self.blue_box_label && label != &self.no_blue_box_label && add && total_chests >= 0 {
            self.add_blue_boxes();
        }

        if label != &self.blue_box_label && label != &self.no_blue_box_label && !add && total_chests > 0 {
            self.subtract_blue_boxes();
        }

        if add {
            total_chests = total_chests + 1;
        } else {
            total_chests = total_chests - 1;
            if total_chests < 0 {
                total_chests = 0;
            }
        }

        set_db_value(&field, &total_chests.to_string());
        label.set_text(&total_chests.to_string());

        self.calculate_coronation_rings();
        self.calculate_lineage_rings();
        self.calculate_intricacy_rings();
        self.calculate_gold_bars();
    }

    fn calculate_droprate(&self, field: &str) {
        let label = match field {
            "coronation_rings" => &self.coronation_ring_label,
            "lineage_rings" => &self.lineage_ring_label,
            "intricacy_rings" => &self.intricacy_ring_label,
            "gold_bars" => &self.gold_bar_label,
            _ => return
        };

        let percentage = match field {
            "coronation_rings" => &self.coronation_ring_percentage,
            "lineage_rings" => &self.lineage_ring_percentage,
            "intricacy_rings" => &self.intricacy_ring_percentage,
            "gold_bars" => &self.gold_bar_percentage,
            _ => return
        };

        let total_blue_chests = &self.blue_box_label.text().parse::<f32>().unwrap();
        let total_no_blue_chests = &self.no_blue_box_label.text().parse::<f32>().unwrap();
        let only_blues = get_settings_value("only_blues");
        let mut total_chests = total_blue_chests + total_no_blue_chests;
        if only_blues == "1" {
            total_chests = *total_blue_chests;
        }
        let dropped_chests = label.text().parse::<f32>().unwrap();

        if dropped_chests >= 0.0 && total_chests >= 1.0 {
            percentage.set_text(&get_percentage(dropped_chests, total_chests));
        } else {
            percentage.set_text("");
        }
    }

    fn calculate_coronation_rings(&self) {
        self.calculate_droprate("coronation_rings");
    }

    fn calculate_lineage_rings(&self) {
        self.calculate_droprate("lineage_rings");
    }

    fn calculate_intricacy_rings(&self) {
        self.calculate_droprate("intricacy_rings");
    }

    fn calculate_gold_bars(&self) {
        self.calculate_droprate("gold_bars");
    }

    // All of this because I can't pass parameters to change_value from the events themselves.
    // Every button gets its own add/subtract event that just calls the real fn with what should be the event params

    fn add_blue_boxes(&self) {
        self.change_value("blue_boxes", true);
    }

    fn subtract_blue_boxes(&self) {
        self.change_value("blue_boxes", false);
    }

    fn add_no_blue_boxes(&self) {
        self.change_value("no_blue_boxes", true);
    }

    fn subtract_no_blue_boxes(&self) {
        self.change_value("no_blue_boxes", false);
    }

    fn add_coronation_rings(&self) {
        self.change_value("coronation_rings", true);
    }

    fn subtract_coronation_rings(&self) {
        self.change_value("coronation_rings", false);
    }

    fn add_lineage_rings(&self) {
        self.change_value("lineage_rings", true);
    }

    fn subtract_lineage_rings(&self) {
        self.change_value("lineage_rings", false);
    }

    fn add_intricacy_rings(&self) {
        self.change_value("intricacy_rings", true);
    }

    fn subtract_intricacy_rings(&self) {
        self.change_value("intricacy_rings", false);
    }

    fn add_gold_bars(&self) {
        self.change_value("gold_bars", true);
    }

    fn subtract_gold_bars(&self) {
        self.change_value("gold_bars", false);
    }

    fn kill_app(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn get_percentage(x: f32, y: f32) -> String {
    let result = (x * 100.0) / y;
    return format!("{:.2}%", result);
}

fn get_db() -> ini::Ini {
    return Ini::load_from_file(Path::new("./db/log.ini")).unwrap();
}

fn save_db(conf: ini::Ini) {
        conf.write_to_file("./db/log.ini").unwrap();
}

fn get_settings_value(field: &str) -> String {
    let mut conf = get_db();
    let val = conf.with_section(Some("settings")).get(field).unwrap().to_string();
    val
}

fn get_db_value(field: &str) -> String {
    let mut conf = get_db();
    let val = conf.with_section(Some("drops")).get(field).unwrap().to_string();
    val
}

fn set_db_value(field: &str, total: &str) {
        let mut conf = get_db();
        conf.with_section(Some("drops")).set(field, total);
        save_db(conf);
}

fn create_db_path() -> std::io::Result<()> {
    fs::create_dir("./db/")?;
    Ok(())
}

fn create_new_log() {
    match create_db_path() {
        Err(e) => println!("Could not create folder: {:?}", e),
        _ => ()
    }
    if !Path::new("./db/log.ini").exists() {
        let mut conf = Ini::new();
        conf.with_section(None::<String>).set("encoding", "utf-8");
        conf.with_section(Some("settings"))
            .set("only_blues", "1");
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

fn main() {
    create_new_log();
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = BarTracker::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
