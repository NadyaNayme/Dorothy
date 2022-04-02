extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwd::NwgUi;

use crate::modules::utility::get_percentage;

static BLUE_CHEST: &[u8] = include_bytes!("./images/blue_chest.ico");
static NO_BLUE_CHEST: &[u8] = include_bytes!("./images/no_blue_chest.ico");
static C_RING: &[u8] = include_bytes!("./images/coronation_ring.ico");
static L_RING: &[u8] = include_bytes!("./images/lineage_ring.ico");
static I_RING: &[u8] = include_bytes!("./images/intricacy_ring.ico");
static STEEL_BAR: &[u8] = include_bytes!("./images/steel_bar.ico");
static GOLD_BAR: &[u8] = include_bytes!("./images/hihi.ico");

#[derive(Default, NwgUi)]
pub struct BarTracker {
    #[nwg_control(size: (330, 400), position: (500, 500), title: "Dorothy", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [BarTracker::kill_app], OnInit: [BarTracker::init])]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(family: "Meiryo", size: 16)]
    font: nwg::Font,

    #[nwg_resource(family: "Meiryo UI", size: 14)]
    smallfont: nwg::Font,

    #[nwg_control(text: "心臓蛇団のスーパーシークレットプログラム", font: Some(&data.font), size: (320, 20), position: (10, 5))]
    instructions: nwg::Label,

    #[nwg_control(text: "Export", font: Some(&data.font), size: (60, 32), position: (265, 32))]
    #[nwg_events(OnButtonClick: [BarTracker::app_export_csv])]
    export: nwg::Button,

    #[nwg_control(text: "Akasha", font: Some(&data.font), size: (60, 32), position: (5, 32))]
    #[nwg_events(OnButtonClick: [BarTracker::show_ui_tab(SELF, CTRL)])]
    akasha_ui_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 80), icon: Some(&nwg::Icon::from_bin(BLUE_CHEST)?))]
    akasha_blue_box: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("akasha_blue_boxes"), font: Some(&data.font), size: (50, 25), position: (48, 87))]
    akasha_blue_box_label: nwg::Label,

    #[nwg_control(size: (32, 32), position: (10, 120), icon: Some(&nwg::Icon::from_bin(NO_BLUE_CHEST)?))]
    akasha_no_blue_box: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("akasha_no_blue_boxes"), font: Some(&data.font), size: (30, 16), position: (48, 127))]
    akasha_no_blue_box_label: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 120))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_no_blue_box_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 160), icon: Some(&nwg::Icon::from_bin(C_RING)?))]
    akasha_coronation_ring: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("akasha_coronation_rings"), font: Some(&data.font), size: (30, 16), position: (48, 160))]
    akasha_coronation_ring_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 174))]
    akasha_coronation_ring_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 160))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_coronation_rings_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 200), icon: Some(&nwg::Icon::from_bin(L_RING)?))]
    akasha_lineage_ring: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("akasha_lineage_rings"), font: Some(&data.font), size: (30, 16), position: (48, 200))]
    akasha_lineage_ring_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 214))]
    akasha_lineage_ring_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 200))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_lineage_rings_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 240), icon: Some(&nwg::Icon::from_bin(I_RING)?))]
    akasha_intricacy_ring: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("akasha_intricacy_rings"), font: Some(&data.font), size: (30, 16), position: (48, 240))]
    akasha_intricacy_ring_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 254))]
    akasha_intricacy_ring_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 240))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_intricacy_rings_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 280), icon: Some(&nwg::Icon::from_bin(GOLD_BAR)?))]
    akasha_gold_bar: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("akasha_gold_bars"), font: Some(&data.font), size: (30, 16), position: (48, 280))]
    akasha_gold_bar_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 294))]
    akasha_gold_bar_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 280))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_gold_bars_button: nwg::Button,

    #[nwg_control(text: "PBHL", font: Some(&data.font), size: (60, 32), position: (70, 32))]
    #[nwg_events(OnButtonClick: [BarTracker::show_ui_tab(SELF, CTRL)])]
    pbhl_ui_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 80), icon: Some(&nwg::Icon::from_bin(BLUE_CHEST)?))]
    pbhl_blue_box: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("pbhl_blue_boxes"), font: Some(&data.font), size: (50, 25), position: (48, 87))]
    pbhl_blue_box_label: nwg::Label,

    #[nwg_control(size: (32, 32), position: (10, 120), icon: Some(&nwg::Icon::from_bin(NO_BLUE_CHEST)?))]
    pbhl_no_blue_box: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("pbhl_no_blue_boxes"), font: Some(&data.font), size: (30, 16), position: (48, 127))]
    pbhl_no_blue_box_label: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 120))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_no_blue_box_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 160), icon: Some(&nwg::Icon::from_bin(C_RING)?))]
    pbhl_coronation_ring: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("pbhl_coronation_rings"), font: Some(&data.font), size: (30, 16), position: (48, 160))]
    pbhl_coronation_ring_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 174))]
    pbhl_coronation_ring_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 160))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_coronation_rings_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 200), icon: Some(&nwg::Icon::from_bin(L_RING)?))]
    pbhl_lineage_ring: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("pbhl_lineage_rings"), font: Some(&data.font), size: (30, 16), position: (48, 200))]
    pbhl_lineage_ring_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 214))]
    pbhl_lineage_ring_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 200))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_lineage_rings_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 240), icon: Some(&nwg::Icon::from_bin(I_RING)?))]
    pbhl_intricacy_ring: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("pbhl_intricacy_rings"), font: Some(&data.font), size: (30, 16), position: (48, 240))]
    pbhl_intricacy_ring_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 254))]
    pbhl_intricacy_ring_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 240))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_intricacy_rings_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 280), icon: Some(&nwg::Icon::from_bin(GOLD_BAR)?))]
    pbhl_gold_bar: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("pbhl_gold_bars"), font: Some(&data.font), size: (30, 16), position: (48, 280))]
    pbhl_gold_bar_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 294))]
    pbhl_gold_bar_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 280))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_gold_bars_button: nwg::Button,

    #[nwg_control(text: "GOHL", font: Some(&data.font), size: (60, 32), position: (135, 32))]
    #[nwg_events(OnButtonClick: [BarTracker::show_ui_tab(SELF, CTRL)])]
    gohl_ui_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 80), icon: Some(&nwg::Icon::from_bin(BLUE_CHEST)?))]
    gohl_blue_box: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("gohl_blue_boxes"), font: Some(&data.font), size: (50, 25), position: (48, 87))]
    gohl_blue_box_label: nwg::Label,

    #[nwg_control(size: (32, 32), position: (10, 120), icon: Some(&nwg::Icon::from_bin(NO_BLUE_CHEST)?))]
    gohl_no_blue_box: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("gohl_no_blue_boxes"), font: Some(&data.font), size: (30, 16), position: (48, 127))]
    gohl_no_blue_box_label: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 120))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_no_blue_box_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 160), icon: Some(&nwg::Icon::from_bin(C_RING)?))]
    gohl_coronation_ring: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("gohl_coronation_rings"), font: Some(&data.font), size: (30, 16), position: (48, 160))]
    gohl_coronation_ring_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 174))]
    gohl_coronation_ring_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 160))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_coronation_rings_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 200), icon: Some(&nwg::Icon::from_bin(L_RING)?))]
    gohl_lineage_ring: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("gohl_lineage_rings"), font: Some(&data.font), size: (30, 16), position: (48, 200))]
    gohl_lineage_ring_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 214))]
    gohl_lineage_ring_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 200))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_lineage_rings_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 240), icon: Some(&nwg::Icon::from_bin(I_RING)?))]
    gohl_intricacy_ring: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("gohl_intricacy_rings"), font: Some(&data.font), size: (30, 16), position: (48, 240))]
    gohl_intricacy_ring_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 254))]
    gohl_intricacy_ring_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 240))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_intricacy_rings_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 280), icon: Some(&nwg::Icon::from_bin(GOLD_BAR)?))]
    gohl_gold_bar: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("gohl_gold_bars"), font: Some(&data.font), size: (30, 16), position: (48, 280))]
    gohl_gold_bar_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 294))]
    gohl_gold_bar_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 280))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_gold_bars_button: nwg::Button,

    #[nwg_control(size: (32, 32), position: (10, 320), icon: Some(&nwg::Icon::from_bin(STEEL_BAR)?))]
    gohl_steel_bar: nwg::ImageFrame,
    #[nwg_control(text: &crate::modules::database::get_db_value("gohl_steel_bars"), font: Some(&data.font), size: (30, 16), position: (48, 320))]
    gohl_steel_bar_label: nwg::Label,

    #[nwg_control(text: "", font: Some(&data.smallfont), size: (50, 16), position: (48, 334))]
    gohl_steel_bar_percentage: nwg::Label,

    #[nwg_control(text: "+1", font: Some(&data.font), size: (60, 32), position: (265, 320))]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_steel_bars_button: nwg::Button,

    #[nwg_control(text: "Hosts", font: Some(&data.font), size: (60, 32), position: (200, 32))]
    #[nwg_events(OnButtonClick: [BarTracker::show_ui_tab(SELF, CTRL)])]
    hosts_ui_button: nwg::Button,
}

impl BarTracker {

    fn init(&self) {
        let em = &self.embed;
        self.window.set_icon(em.icon_str("DOROTHY", None).as_ref());
        self.show_ui_tab(&self.pbhl_ui_button);
        self.calculate_droprates();
    }

    fn akasha_ui(&self, v: bool) {
            self.akasha_ui_button.set_enabled(!v);
            self.akasha_blue_box_label.set_visible(v);
            self.akasha_blue_box.set_visible(v);
            self.akasha_no_blue_box.set_visible(v);
            self.akasha_no_blue_box_label.set_visible(v);
            self.akasha_no_blue_box_button.set_visible(v);
            self.akasha_coronation_ring.set_visible(v);
            self.akasha_coronation_ring_label.set_visible(v);
            self.akasha_coronation_ring_percentage.set_visible(v);
            self.akasha_coronation_rings_button.set_visible(v);
            self.akasha_lineage_ring.set_visible(v);
            self.akasha_lineage_ring_label.set_visible(v);
            self.akasha_lineage_ring_percentage.set_visible(v);
            self.akasha_lineage_rings_button.set_visible(v);
            self.akasha_intricacy_ring.set_visible(v);
            self.akasha_intricacy_ring_label.set_visible(v);
            self.akasha_intricacy_ring_percentage.set_visible(v);
            self.akasha_intricacy_rings_button.set_visible(v);
            self.akasha_gold_bar.set_visible(v);
            self.akasha_gold_bar_label.set_visible(v);
            self.akasha_gold_bar_percentage.set_visible(v);
            self.akasha_gold_bars_button.set_visible(v);
    }

    fn pbhl_ui(&self, v: bool) {
            self.pbhl_ui_button.set_enabled(!v);
            self.pbhl_blue_box_label.set_visible(v);
            self.pbhl_blue_box.set_visible(v);
            self.pbhl_no_blue_box.set_visible(v);
            self.pbhl_no_blue_box_label.set_visible(v);
            self.pbhl_no_blue_box_button.set_visible(v);
            self.pbhl_coronation_ring.set_visible(v);
            self.pbhl_coronation_ring_label.set_visible(v);
            self.pbhl_coronation_ring_percentage.set_visible(v);
            self.pbhl_coronation_rings_button.set_visible(v);
            self.pbhl_lineage_ring.set_visible(v);
            self.pbhl_lineage_ring_label.set_visible(v);
            self.pbhl_lineage_ring_percentage.set_visible(v);
            self.pbhl_lineage_rings_button.set_visible(v);
            self.pbhl_intricacy_ring.set_visible(v);
            self.pbhl_intricacy_ring_label.set_visible(v);
            self.pbhl_intricacy_ring_percentage.set_visible(v);
            self.pbhl_intricacy_rings_button.set_visible(v);
            self.pbhl_gold_bar.set_visible(v);
            self.pbhl_gold_bar_label.set_visible(v);
            self.pbhl_gold_bar_percentage.set_visible(v);
            self.pbhl_gold_bars_button.set_visible(v);
    }

    fn gohl_ui(&self, v: bool) {
            self.gohl_ui_button.set_enabled(!v);
            self.gohl_blue_box_label.set_visible(v);
            self.gohl_blue_box.set_visible(v);
            self.gohl_no_blue_box.set_visible(v);
            self.gohl_no_blue_box_label.set_visible(v);
            self.gohl_no_blue_box_button.set_visible(v);
            self.gohl_coronation_ring.set_visible(v);
            self.gohl_coronation_ring_label.set_visible(v);
            self.gohl_coronation_ring_percentage.set_visible(v);
            self.gohl_coronation_rings_button.set_visible(v);
            self.gohl_lineage_ring.set_visible(v);
            self.gohl_lineage_ring_label.set_visible(v);
            self.gohl_lineage_ring_percentage.set_visible(v);
            self.gohl_lineage_rings_button.set_visible(v);
            self.gohl_intricacy_ring.set_visible(v);
            self.gohl_intricacy_ring_label.set_visible(v);
            self.gohl_intricacy_ring_percentage.set_visible(v);
            self.gohl_intricacy_rings_button.set_visible(v);
            self.gohl_gold_bar.set_visible(v);
            self.gohl_gold_bar_label.set_visible(v);
            self.gohl_gold_bar_percentage.set_visible(v);
            self.gohl_gold_bars_button.set_visible(v);
            self.gohl_steel_bar.set_visible(v);
            self.gohl_steel_bar_label.set_visible(v);
            self.gohl_steel_bar_percentage.set_visible(v);
            self.gohl_steel_bars_button.set_visible(v);
    }

    fn show_ui_tab(&self, btn: &nwg::Button) {
        if btn == &self.akasha_ui_button {
            self.akasha_ui(true);
            self.pbhl_ui(false);
            self.gohl_ui(false);
            self.hosts_ui_button.set_visible(false);
        } else if btn == &self.pbhl_ui_button {
            self.akasha_ui(false);
            self.pbhl_ui(true);
            self.gohl_ui(false);
            self.hosts_ui_button.set_visible(false);
        } else if btn == &self.gohl_ui_button {
            self.akasha_ui(false);
            self.pbhl_ui(false);
            self.gohl_ui(true);
            self.hosts_ui_button.set_visible(false);
        } else if btn == &self.hosts_ui_button {
            self.akasha_ui(false);
            self.pbhl_ui(false);
            self.gohl_ui(false);
        }
    }

    fn reset_labels(&self) {
        self.akasha_blue_box_label.set_text("0");
        self.akasha_no_blue_box_label.set_text("0");
        self.akasha_coronation_ring_label.set_text("0");
        self.akasha_lineage_ring_label.set_text("0");
        self.akasha_intricacy_ring_label.set_text("0");
        self.akasha_gold_bar_label.set_text("0");

        self.pbhl_blue_box_label.set_text("0");
        self.pbhl_no_blue_box_label.set_text("0");
        self.pbhl_coronation_ring_label.set_text("0");
        self.pbhl_lineage_ring_label.set_text("0");
        self.pbhl_intricacy_ring_label.set_text("0");
        self.pbhl_gold_bar_label.set_text("0");

        self.gohl_blue_box_label.set_text("0");
        self.gohl_no_blue_box_label.set_text("0");
        self.gohl_coronation_ring_label.set_text("0");
        self.gohl_lineage_ring_label.set_text("0");
        self.gohl_intricacy_ring_label.set_text("0");
        self.gohl_steel_bar_label.set_text("0");
        self.gohl_gold_bar_label.set_text("0");

        // self.hosts_pbhl_bar_label.set_text("0");
        // self.hosts_ubhl_bar_label.set_text("0");
        // self.hosts_xeno_bar_label.set_text("0");
    }

    fn change_value(&self, field: &str, add: bool) {
        let label = match field {
            "akasha_blue_boxes" => &self.akasha_blue_box_label,
            "akasha_no_blue_boxes" => &self.akasha_no_blue_box_label,
            "akasha_coronation_rings" => &self.akasha_coronation_ring_label,
            "akasha_lineage_rings" => &self.akasha_lineage_ring_label,
            "akasha_intricacy_rings" => &self.akasha_intricacy_ring_label,
            "akasha_gold_bars" => &self.akasha_gold_bar_label,
            "pbhl_blue_boxes" => &self.pbhl_blue_box_label,
            "pbhl_no_blue_boxes" => &self.pbhl_no_blue_box_label,
            "pbhl_coronation_rings" => &self.pbhl_coronation_ring_label,
            "pbhl_lineage_rings" => &self.pbhl_lineage_ring_label,
            "pbhl_intricacy_rings" => &self.pbhl_intricacy_ring_label,
            "pbhl_gold_bars" => &self.pbhl_gold_bar_label,
            "gohl_blue_boxes" => &self.gohl_blue_box_label,
            "gohl_no_blue_boxes" => &self.gohl_no_blue_box_label,
            "gohl_coronation_rings" => &self.gohl_coronation_ring_label,
            "gohl_lineage_rings" => &self.gohl_lineage_ring_label,
            "gohl_intricacy_rings" => &self.gohl_intricacy_ring_label,
            "gohl_gold_bars" => &self.gohl_gold_bar_label,
            "gohl_steel_bars" => &self.gohl_steel_bar_label,
            _ => &self.pbhl_blue_box_label,
        };

        let mut total_chests = label.text().parse::<i32>().unwrap();

        if (label == &self.akasha_coronation_ring_label
            || label == &self.akasha_lineage_ring_label
            || label == &self.akasha_intricacy_ring_label
            || label == &self.akasha_gold_bar_label)
            && add
            && total_chests >= 0
        {
            self.add_blue_boxes("akasha");
        }

        if (label == &self.akasha_coronation_ring_label
            || label == &self.akasha_lineage_ring_label
            || label == &self.akasha_intricacy_ring_label
            || label == &self.akasha_gold_bar_label)
            && !add
            && total_chests > 0
        {
            self.subtract_blue_boxes("akasha");
        }

        if (label == &self.pbhl_coronation_ring_label
            || label == &self.pbhl_lineage_ring_label
            || label == &self.pbhl_intricacy_ring_label
            || label == &self.pbhl_gold_bar_label)
            && add
            && total_chests >= 0
        {
            self.add_blue_boxes("pbhl");
        }

        if (label == &self.pbhl_coronation_ring_label
            || label == &self.pbhl_lineage_ring_label
            || label == &self.pbhl_intricacy_ring_label
            || label == &self.pbhl_gold_bar_label)
            && !add
            && total_chests > 0
        {
            self.subtract_blue_boxes("pbhl");
        }

        if (label == &self.gohl_coronation_ring_label
            || label == &self.gohl_lineage_ring_label
            || label == &self.gohl_intricacy_ring_label
            || label == &self.gohl_gold_bar_label
            || label == &self.gohl_steel_bar_label)
            && add
            && total_chests >= 0
        {
            self.add_blue_boxes("gohl");
        }

        if (label == &self.gohl_coronation_ring_label
            || label == &self.gohl_lineage_ring_label
            || label == &self.gohl_intricacy_ring_label
            || label == &self.gohl_gold_bar_label
            || label == &self.gohl_steel_bar_label)
            && !add
            && total_chests > 0
        {
            self.subtract_blue_boxes("gohl");
        }

        if add {
            total_chests += 1;
        } else {
            total_chests -= 1;
            if total_chests < 0 {
                total_chests = 0;
            }
        }

        crate::modules::database::set_db_value(field, &total_chests.to_string());
        label.set_text(&total_chests.to_string());

        self.calculate_droprates();
    }

    fn calculate_droprate(&self, field: &str) {
        let label = match field {
            "akasha_coronation_rings" => &self.akasha_coronation_ring_label,
            "akasha_lineage_rings" => &self.akasha_lineage_ring_label,
            "akasha_intricacy_rings" => &self.akasha_intricacy_ring_label,
            "akasha_gold_bars" => &self.akasha_gold_bar_label,
            "pbhl_coronation_rings" => &self.pbhl_coronation_ring_label,
            "pbhl_lineage_rings" => &self.pbhl_lineage_ring_label,
            "pbhl_intricacy_rings" => &self.pbhl_intricacy_ring_label,
            "pbhl_gold_bars" => &self.pbhl_gold_bar_label,
            "gohl_coronation_rings" => &self.gohl_coronation_ring_label,
            "gohl_lineage_rings" => &self.gohl_lineage_ring_label,
            "gohl_intricacy_rings" => &self.gohl_intricacy_ring_label,
            "gohl_gold_bars" => &self.gohl_gold_bar_label,
            "gohl_steel_bars" => &self.gohl_steel_bar_label,
            _ => return,
        };

        let percentage = match field {
            "akasha_coronation_rings" => &self.akasha_coronation_ring_percentage,
            "akasha_lineage_rings" => &self.akasha_lineage_ring_percentage,
            "akasha_intricacy_rings" => &self.akasha_intricacy_ring_percentage,
            "akasha_gold_bars" => &self.akasha_gold_bar_percentage,
            "pbhl_coronation_rings" => &self.pbhl_coronation_ring_percentage,
            "pbhl_lineage_rings" => &self.pbhl_lineage_ring_percentage,
            "pbhl_intricacy_rings" => &self.pbhl_intricacy_ring_percentage,
            "pbhl_gold_bars" => &self.pbhl_gold_bar_percentage,
            "gohl_coronation_rings" => &self.gohl_coronation_ring_percentage,
            "gohl_lineage_rings" => &self.gohl_lineage_ring_percentage,
            "gohl_intricacy_rings" => &self.gohl_intricacy_ring_percentage,
            "gohl_gold_bars" => &self.gohl_gold_bar_percentage,
            "gohl_steel_bars" => &self.gohl_steel_bar_percentage,
            _ => return,
        };

        let total_akasha_blue_chests = &self.akasha_blue_box_label.text().parse::<f32>().unwrap();
        let total_akasha_no_blue_chests =
            &self.akasha_no_blue_box_label.text().parse::<f32>().unwrap();
        let total_pbhl_blue_chests = &self.pbhl_blue_box_label.text().parse::<f32>().unwrap();
        let total_pbhl_no_blue_chests = &self.pbhl_no_blue_box_label.text().parse::<f32>().unwrap();
        let total_gohl_blue_chests = &self.gohl_blue_box_label.text().parse::<f32>().unwrap();
        let total_gohl_no_blue_chests = &self.gohl_no_blue_box_label.text().parse::<f32>().unwrap();
        let only_blues = crate::modules::database::get_settings_value("only_blues");
        let mut total_akasha_chests = total_akasha_blue_chests + total_akasha_no_blue_chests;
        let mut total_pbhl_chests = total_pbhl_blue_chests + total_pbhl_no_blue_chests;
        let mut total_gohl_chests = total_gohl_blue_chests + total_gohl_no_blue_chests;
        if only_blues == "1" {
            total_pbhl_chests = *total_pbhl_blue_chests;
            total_akasha_chests = *total_akasha_blue_chests;
            total_gohl_chests = *total_gohl_blue_chests;
        }
        let dropped_chests = label.text().parse::<f32>().unwrap();

        if total_akasha_blue_chests >= &0.0 && total_akasha_chests >= 1.0 {
            percentage.set_text(&get_percentage(dropped_chests, total_akasha_chests));
        }
        if total_pbhl_blue_chests >= &0.0 && total_pbhl_chests >= 1.0 {
            percentage.set_text(&get_percentage(dropped_chests, total_pbhl_chests));
        }
        if total_gohl_blue_chests >= &0.0 && total_gohl_chests >= 1.0 {
            percentage.set_text(&get_percentage(dropped_chests, total_gohl_chests));
        }
    }

    fn calculate_droprates(&self) {
        self.calculate_droprate("akasha_coronation_rings");
        self.calculate_droprate("akasha_lineage_rings");
        self.calculate_droprate("akasha_intricacy_rings");
        self.calculate_droprate("akasha_gold_bars");
        self.calculate_droprate("pbhl_coronation_rings");
        self.calculate_droprate("pbhl_lineage_rings");
        self.calculate_droprate("pbhl_intricacy_rings");
        self.calculate_droprate("pbhl_gold_bars");
        self.calculate_droprate("gohl_coronation_rings");
        self.calculate_droprate("gohl_lineage_rings");
        self.calculate_droprate("gohl_intricacy_rings");
        self.calculate_droprate("gohl_gold_bars");
        self.calculate_droprate("gohl_steel_bars");
    }

    fn add_blue_boxes(&self, raid: &str) {
        if raid == "akasha" {
            self.change_value("akasha_blue_boxes", true);
        } else if raid == "pbhl" {
            self.change_value("pbhl_blue_boxes", true);
        } else if raid == "gohl" {
            self.change_value("gohl_blue_boxes", true);
        }
    }

    fn subtract_blue_boxes(&self, raid: &str) {
        if raid == "akasha" {
            self.change_value("akasha_blue_boxes", false);
        } else if raid == "pbhl" {
            self.change_value("pbhl_blue_boxes", false);
        } else if raid == "gohl" {
            self.change_value("gohl_blue_boxes", false);
        }
    }

    fn button_actions(&self, btn: &nwg::Button, event: nwg::Event) {
        use nwg::Event as E;
        use nwg::MousePressEvent::*;

        match event {
            E::OnMousePress(MousePressLeftDown) => {
                if btn == &self.akasha_no_blue_box_button {
                    self.change_value("akasha_no_blue_boxes", true);
                } else if btn == &self.akasha_coronation_rings_button {
                    self.change_value("akasha_coronation_rings", true);
                } else if btn == &self.akasha_lineage_rings_button {
                    self.change_value("akasha_lineage_rings", true);
                } else if btn == &self.akasha_intricacy_rings_button {
                    self.change_value("akasha_intricacy_rings", true);
                } else if btn == &self.akasha_gold_bars_button {
                    self.change_value("akasha_gold_bars", true);
                } else if btn == &self.pbhl_no_blue_box_button {
                    self.change_value("pbhl_no_blue_boxes", true);
                } else if btn == &self.pbhl_coronation_rings_button {
                    self.change_value("pbhl_coronation_rings", true);
                } else if btn == &self.pbhl_lineage_rings_button {
                    self.change_value("pbhl_lineage_rings", true);
                } else if btn == &self.pbhl_intricacy_rings_button {
                    self.change_value("pbhl_intricacy_rings", true);
                } else if btn == &self.pbhl_gold_bars_button {
                    self.change_value("pbhl_gold_bars", true);
                } else if btn == &self.gohl_no_blue_box_button {
                    self.change_value("gohl_no_blue_boxes", true);
                } else if btn == &self.gohl_coronation_rings_button {
                    self.change_value("gohl_coronation_rings", true);
                } else if btn == &self.gohl_lineage_rings_button {
                    self.change_value("gohl_lineage_rings", true);
                } else if btn == &self.gohl_intricacy_rings_button {
                    self.change_value("gohl_intricacy_rings", true);
                } else if btn == &self.gohl_gold_bars_button {
                    self.change_value("gohl_gold_bars", true);
                } else if btn == &self.gohl_steel_bars_button {
                    self.change_value("gohl_steel_bars", true);
                }
            }
            E::OnMousePress(MousePressRightDown) => {
                if btn == &self.akasha_no_blue_box_button {
                    self.change_value("akasha_no_blue_boxes", false);
                } else if btn == &self.akasha_coronation_rings_button {
                    self.change_value("akasha_coronation_rings", false);
                } else if btn == &self.akasha_lineage_rings_button {
                    self.change_value("akasha_lineage_rings", false);
                } else if btn == &self.akasha_intricacy_rings_button {
                    self.change_value("akasha_intricacy_rings", false);
                } else if btn == &self.akasha_gold_bars_button {
                    self.change_value("akasha_gold_bars", false);
                } else if btn == &self.pbhl_no_blue_box_button {
                    self.change_value("pbhl_no_blue_boxes", false);
                } else if btn == &self.pbhl_coronation_rings_button {
                    self.change_value("pbhl_coronation_rings", false);
                } else if btn == &self.pbhl_lineage_rings_button {
                    self.change_value("pbhl_lineage_rings", false);
                } else if btn == &self.pbhl_intricacy_rings_button {
                    self.change_value("pbhl_intricacy_rings", false);
                } else if btn == &self.pbhl_gold_bars_button {
                    self.change_value("pbhl_gold_bars", false);
                } else if btn == &self.gohl_no_blue_box_button {
                    self.change_value("gohl_no_blue_boxes", false);
                } else if btn == &self.gohl_coronation_rings_button {
                    self.change_value("gohl_coronation_rings", false);
                } else if btn == &self.gohl_lineage_rings_button {
                    self.change_value("gohl_lineage_rings", false);
                } else if btn == &self.gohl_intricacy_rings_button {
                    self.change_value("gohl_intricacy_rings", false);
                } else if btn == &self.gohl_gold_bars_button {
                    self.change_value("gohl_gold_bars", false);
                } else if btn == &self.gohl_steel_bars_button {
                    self.change_value("gohl_steel_bars", false);
                }
            }
            _ => unreachable!(),
        }
    }

    fn app_export_csv(&self) {
        let _ = &crate::modules::csv::export_csv();
        let no_reset = crate::modules::database::get_settings_value("no_reset");
        if no_reset == "0" {
            let _ = self.reset_labels();
            let _ = self.calculate_droprates();
        }
    }

    fn kill_app(&self) {
        nwg::stop_thread_dispatch();
    }
}
