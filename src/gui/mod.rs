extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwd::NwgUi;

use crate::modules::*;

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
    #[nwg_control(text: &mut crate::modules::database::get_db_value("blue_boxes"), font: Some(&data.font), size: (50, 25), position: (48, 47))]
    blue_box_label: nwg::Label,

    #[nwg_control(size: (32, 32), position: (10, 80), icon: Some(&nwg::Icon::from_bin(NO_BLUE_CHEST)?))]
    no_blue_box: nwg::ImageFrame,
    #[nwg_control(text: &mut crate::modules::database::get_db_value("no_blue_boxes"), font: Some(&data.font), size: (30, 16), position: (48, 87))]
    no_blue_box_label: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (32, 32), position: (225, 80))]
    #[nwg_events( OnButtonClick: [BarTracker::add_no_blue_boxes], OnButtonDoubleClick: [BarTracker::add_no_blue_boxes])]
    no_blue_box_add_button: nwg::Button,

    #[nwg_control(text: "-1", font: Some(&data.font), size: (32, 32), position: (260, 80))]
    #[nwg_events(OnButtonClick: [BarTracker::subtract_no_blue_boxes], OnButtonDoubleClick: [BarTracker::subtract_no_blue_boxes])]
    no_blue_box_subtract_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 120), icon: Some(&nwg::Icon::from_bin(C_RING)?))]
    coronation_ring: nwg::ImageFrame,
    #[nwg_control(text: &mut crate::modules::database::get_db_value("coronation_rings"), font: Some(&data.font), size: (30, 16), position: (48, 120))]
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
    #[nwg_control(text: &mut crate::modules::database::get_db_value("lineage_rings"), font: Some(&data.font), size: (30, 16), position: (48, 160))]
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
    #[nwg_control(text: &mut crate::modules::database::get_db_value("intricacy_rings"), font: Some(&data.font), size: (30, 16), position: (48, 200))]
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
    #[nwg_control(text: &mut crate::modules::database::get_db_value("gold_bars"), font: Some(&data.font), size: (30, 16), position: (48, 240))]
    gold_bar_label: nwg::Label,

    #[nwg_control(text: "TBD", font: Some(&data.smallfont), size: (50, 16), position: (48, 254))]
    gold_bar_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (32, 32), position: (225, 240))]
    #[nwg_events( OnButtonClick: [BarTracker::add_gold_bars], OnButtonDoubleClick: [BarTracker::add_gold_bars])]
    gold_bars_add_button: nwg::Button,

    #[nwg_control(text: "-1", font: Some(&data.font), size: (32, 32), position: (260, 240))]
    #[nwg_events(OnButtonClick: [BarTracker::subtract_gold_bars], OnButtonDoubleClick: [BarTracker::subtract_gold_bars])]
    gold_bars_subtract_button: nwg::Button,

    #[nwg_control(text: "Export", font: Some(&data.font), size: (60, 32), position: (230, 30))]
    #[nwg_events(OnButtonClick: [BarTracker::app_export_csv])]
    export: nwg::Button,
}

impl BarTracker {
    fn init(&self) {
        let em = &self.embed;
        self.window.set_icon(em.icon_str("DOROTHY", None).as_ref());
    }

    fn reset_labels(&self) {
        self.blue_box_label.set_text("0");
        self.no_blue_box_label.set_text("0");
        self.coronation_ring_label.set_text("0");
        self.lineage_ring_label.set_text("0");
        self.intricacy_ring_label.set_text("0");
        self.gold_bar_label.set_text("0");
    }

    fn change_value(&self, field: &str, add: bool) {
        let label = match field {
            "no_blue_boxes" => &self.no_blue_box_label,
            "coronation_rings" => &self.coronation_ring_label,
            "lineage_rings" => &self.lineage_ring_label,
            "intricacy_rings" => &self.intricacy_ring_label,
            "gold_bars" => &self.gold_bar_label,
            _ => &self.blue_box_label,
        };

        let mut total_chests = label.text().parse::<i32>().unwrap();

        if label != &self.blue_box_label
            && label != &self.no_blue_box_label
            && add
            && total_chests >= 0
        {
            self.add_blue_boxes();
        }

        if label != &self.blue_box_label
            && label != &self.no_blue_box_label
            && !add
            && total_chests > 0
        {
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

        crate::modules::database::set_db_value(&field, &total_chests.to_string());
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
            _ => return,
        };

        let percentage = match field {
            "coronation_rings" => &self.coronation_ring_percentage,
            "lineage_rings" => &self.lineage_ring_percentage,
            "intricacy_rings" => &self.intricacy_ring_percentage,
            "gold_bars" => &self.gold_bar_percentage,
            _ => return,
        };

        let total_blue_chests = &self.blue_box_label.text().parse::<f32>().unwrap();
        let total_no_blue_chests = &self.no_blue_box_label.text().parse::<f32>().unwrap();
        let only_blues = crate::modules::database::get_settings_value("only_blues");
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

    fn app_export_csv(&self) {
        let _ = &crate::modules::csv::export_csv();
        let no_reset = crate::modules::database::get_settings_value("no_reset");
        if no_reset == "0" {
            let _ = self.reset_labels();
            let _ = self.calculate_coronation_rings();
            let _ = self.calculate_lineage_rings();
            let _ = self.calculate_intricacy_rings();
            let _ = self.calculate_gold_bars();
        }
    }

    fn kill_app(&self) {
        nwg::stop_thread_dispatch();
    }
}
