extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
extern crate winapi;
use nwd::NwgUi;
use nwg::CheckBoxState;
use std::rc::Rc;

use crate::modules::utility::get_percentage;

static BLUE_CHEST: &[u8] = include_bytes!("./images/blue_chest.ico");
static NO_BLUE_CHEST: &[u8] = include_bytes!("./images/no_blue_chest.ico");
static C_RING: &[u8] = include_bytes!("./images/coronation_ring.ico");
static L_RING: &[u8] = include_bytes!("./images/lineage_ring.ico");
static I_RING: &[u8] = include_bytes!("./images/intricacy_ring.ico");
static AKASHA_TRASH: &[u8] = include_bytes!("./images/akasha_trash.ico");
static GOHL_TRASH: &[u8] = include_bytes!("./images/gohl_trash.ico");
static GOLD_BAR: &[u8] = include_bytes!("./images/hihi.ico");

enum Raids {
    Akasha,
    Pbhl,
    Gohl,
    Ubhl,
    Xeno,
    Event,
}

#[derive(Default, NwgUi)]
pub struct BarTracker {
    #[nwg_control(size: (450, 225), position: (500, 500), title: "Dorothy", flags: "WINDOW|VISIBLE", center:true)]
    #[nwg_events(OnWindowClose: [BarTracker::kill_app], OnInit: [BarTracker::init])]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(family: "Meiryo", size: 16)]
    font: nwg::Font,

    #[nwg_resource(family: "Meiryo UI", size: 14)]
    smallfont: nwg::Font,

    #[nwg_control(parent: Bartracker::window, size: (450, 225), font: Some(&data.smallfont))]
    tabs_container: nwg::TabsContainer,

    #[nwg_control(text: "Settings", parent: BarTracker::tabs_container)]
    settings_tab: nwg::Tab,

    #[nwg_control(text: "Akasha", parent: BarTracker::tabs_container)]
    akasha_tab: nwg::Tab,

    #[nwg_control(text: "PBHL", parent: BarTracker::tabs_container)]
    pbhl_tab: nwg::Tab,

    #[nwg_control(text: "GOHL", parent: BarTracker::tabs_container)]
    gohl_tab: nwg::Tab,

    #[nwg_layout(parent: BarTracker::tabs_container, margin: [10, 10, 10, 10], spacing: 2, min_size: [450, 200], max_size: [520, 200])]
    grid_layout: nwg::GridLayout,

    #[nwg_control(parent: BarTracker::settings_tab, text: " 心臓蛇団のスーパーシークレットプログラム", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 3, col_span: 4)]
    instructions: nwg::Label,

    #[nwg_control(parent: BarTracker::settings_tab, text: "Export", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 0, col_span: 2)]
    #[nwg_events(OnButtonClick: [BarTracker::app_export_csv])]
    export: nwg::Button,

    #[nwg_control(parent: BarTracker::settings_tab, text: "Always On Top", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 0, col_span: 4)]
    #[nwg_events(OnButtonClick: [BarTracker::set_topmost])]
    always_on_top_checkbox: nwg::CheckBox,

    #[nwg_control(parent: BarTracker::settings_tab, text: "Reset Counts on Export", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 1, col_span: 4)]
    #[nwg_events(OnButtonClick: [BarTracker::set_export_reset])]
    export_reset_checkbox: nwg::CheckBox,

    #[nwg_control(parent: BarTracker::settings_tab, text: "Percentage of Total Chests", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 2, col_span: 4)]
    #[nwg_events(OnButtonClick: [BarTracker::set_only_blues])]
    percentage_count_checkbox: nwg::CheckBox,

    #[nwg_control(parent: BarTracker::akasha_tab, icon: Some(&nwg::Icon::from_bin(BLUE_CHEST)?))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 0)]
    akasha_blue_box: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::akasha_tab, icon: Some(&nwg::Icon::from_bin(NO_BLUE_CHEST)?))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 0)]
    akasha_no_blue_box: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::akasha_tab, icon: Some(&nwg::Icon::from_bin(C_RING)?))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 0)]
    akasha_coronation_ring: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::akasha_tab, icon: Some(&nwg::Icon::from_bin(L_RING)?))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 0)]
    akasha_lineage_ring: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::akasha_tab, icon: Some(&nwg::Icon::from_bin(I_RING)?))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 0)]
    akasha_intricacy_ring: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::akasha_tab, icon: Some(&nwg::Icon::from_bin(GOLD_BAR)?))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 0)]
    akasha_gold_bar: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::akasha_tab, icon: Some(&nwg::Icon::from_bin(AKASHA_TRASH)?))]
    #[nwg_layout_item(layout: grid_layout, col: 6, row: 0)]
    akasha_trash: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("akasha_blue_boxes"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 1)]
    akasha_blue_box_label: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("akasha_no_blue_boxes"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 1)]
    akasha_no_blue_box_label: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("akasha_coronation_rings"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 1)]
    akasha_coronation_ring_label: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("akasha_lineage_rings"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 1)]
    akasha_lineage_ring_label: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("akasha_intricacy_rings"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 1)]
    akasha_intricacy_ring_label: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("akasha_gold_bars"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 1)]
    akasha_gold_bar_label: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("akasha_trash"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 6, row: 1)]
    akasha_trash_label: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 2)]
    akasha_coronation_ring_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 2)]
    akasha_lineage_ring_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 2)]
    akasha_intricacy_ring_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 2)]
    akasha_gold_bar_percentage: nwg::Label,


    #[nwg_control(parent: BarTracker::akasha_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 6, row: 2)]
    akasha_trash_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_no_blue_box_button: nwg::Button,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_coronation_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_lineage_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_intricacy_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_gold_bars_button: nwg::Button,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 6, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_trash_button: nwg::Button,

    #[nwg_control(parent: BarTracker::pbhl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(BLUE_CHEST)?))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 0)]
    pbhl_blue_box: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::pbhl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(NO_BLUE_CHEST)?))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 0)]
    pbhl_no_blue_box: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::pbhl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(C_RING)?))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 0)]
    pbhl_coronation_ring: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::pbhl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(L_RING)?))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 0)]
    pbhl_lineage_ring: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::pbhl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(I_RING)?))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 0)]
    pbhl_intricacy_ring: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::pbhl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(GOLD_BAR)?))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 0)]
    pbhl_gold_bar: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::pbhl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("pbhl_blue_boxes"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 1)]
    pbhl_blue_box_label: nwg::Label,

    #[nwg_control(parent: BarTracker::pbhl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("pbhl_no_blue_boxes"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 1)]
    pbhl_no_blue_box_label: nwg::Label,

    #[nwg_control(parent: BarTracker::pbhl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("pbhl_coronation_rings"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 1)]
    pbhl_coronation_ring_label: nwg::Label,

    #[nwg_control(parent: BarTracker::pbhl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("pbhl_lineage_rings"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 1)]
    pbhl_lineage_ring_label: nwg::Label,

    #[nwg_control(parent: BarTracker::pbhl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("pbhl_intricacy_rings"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 1)]
    pbhl_intricacy_ring_label: nwg::Label,

    #[nwg_control(parent: BarTracker::pbhl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("pbhl_gold_bars"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 1)]
    pbhl_gold_bar_label: nwg::Label,

    #[nwg_control(parent: BarTracker::pbhl_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 2)]
    pbhl_coronation_ring_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::pbhl_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 2)]
    pbhl_lineage_ring_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::pbhl_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 2)]
    pbhl_intricacy_ring_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::pbhl_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 2)]
    pbhl_gold_bar_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::pbhl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_no_blue_box_button: nwg::Button,

    #[nwg_control(parent: BarTracker::pbhl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_coronation_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::pbhl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_lineage_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::pbhl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_intricacy_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::pbhl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_gold_bars_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(BLUE_CHEST)?))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 0)]
    gohl_blue_box: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::gohl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(NO_BLUE_CHEST)?))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 0)]
    gohl_no_blue_box: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::gohl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(C_RING)?))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 0)]
    gohl_coronation_ring: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::gohl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(L_RING)?))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 0)]
    gohl_lineage_ring: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::gohl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(I_RING)?))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 0)]
    gohl_intricacy_ring: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::gohl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(GOLD_BAR)?))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 0)]
    gohl_gold_bar: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::gohl_tab, size: (32, 32), icon: Some(&nwg::Icon::from_bin(GOHL_TRASH)?))]
    #[nwg_layout_item(layout: grid_layout, col: 6, row: 0)]
    gohl_trash: nwg::ImageFrame,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("gohl_blue_boxes"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 1)]
    gohl_blue_box_label: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("gohl_no_blue_boxes"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 1)]
    gohl_no_blue_box_label: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("gohl_coronation_rings"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 1)]
    gohl_coronation_ring_label: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("gohl_lineage_rings"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 1)]
    gohl_lineage_ring_label: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("gohl_intricacy_rings"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 1)]
    gohl_intricacy_ring_label: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("gohl_gold_bars"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 1)]
    gohl_gold_bar_label: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: &crate::modules::database::get_db_value("gohl_trash"), font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 6, row: 1)]
    gohl_trash_label: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 2)]
    gohl_coronation_ring_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 2)]
    gohl_lineage_ring_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 2)]
    gohl_intricacy_ring_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 2)]
    gohl_gold_bar_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.smallfont))]
    #[nwg_layout_item(layout: grid_layout, col: 6, row: 2)]
    gohl_trash_percentage: nwg::Label,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_no_blue_box_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_coronation_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_lineage_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_intricacy_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_gold_bars_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 6, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL,EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_trash_button: nwg::Button,
}

impl BarTracker {
    fn init(&self) {
        let em = &self.embed;
        self.window.set_icon(em.icon_str("DOROTHY", None).as_ref());
        self.calculate_droprates();
        let last_active_tab = crate::modules::database::get_settings_value("last_active_tab").parse::<usize>().unwrap_or_default();
        self.tabs_container.set_selected_tab(last_active_tab);
        self.init_settings();
    }

    fn init_settings(&self) {
        let app_rc = Rc::new(self);
        let topmost_setting = crate::modules::database::get_settings_value("always_on_top");
        let only_blues_setting = crate::modules::database::get_settings_value("only_blues");
        let export_reset_setting = crate::modules::database::get_settings_value("reset_on_export");
        if topmost_setting == String::from("1") {
            use winapi::um::winuser::SetWindowPos;
            use winapi::um::winuser::{HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE};
            self.always_on_top_checkbox.set_check_state(CheckBoxState::Checked);
            unsafe {
                SetWindowPos(
                    app_rc.window.handle.hwnd().unwrap(),
                    HWND_TOPMOST,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE,
                );
            };
        }
        if only_blues_setting == String::from("1") {
            self.percentage_count_checkbox.set_check_state(CheckBoxState::Checked);
        }
        if export_reset_setting == String::from("1") {
            self.export_reset_checkbox.set_check_state(CheckBoxState::Checked);
        }
    }

    fn set_topmost(app_rc: &Rc<BarTracker>) {
        use winapi::um::winuser::SetWindowPos;
        use winapi::um::winuser::{HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE};

        let topmost_setting = crate::modules::database::get_settings_value("always_on_top");
        if topmost_setting == String::from("1") {
            unsafe {
                SetWindowPos(
                    app_rc.window.handle.hwnd().unwrap(),
                    HWND_NOTOPMOST,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE,
                );
            };
            crate::modules::database::set_settings_value("always_on_top", "0");
        } else {
            unsafe {
                SetWindowPos(
                    app_rc.window.handle.hwnd().unwrap(),
                    HWND_TOPMOST,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE,
                );
            };
            crate::modules::database::set_settings_value("always_on_top", "1");
        }
    }

    fn set_only_blues(&self) {
        let only_blues_setting = crate::modules::database::get_settings_value("only_blues");
        if only_blues_setting == String::from("1") {
            crate::modules::database::set_settings_value("only_blues", "0");
            self.calculate_droprates();
        } else {
            crate::modules::database::set_settings_value("only_blues", "1");
            self.calculate_droprates();
        }
    }

    fn set_export_reset(&self) {
        let export_reset_setting = crate::modules::database::get_settings_value("reset_on_export");
        if export_reset_setting == String::from("1") {
            crate::modules::database::set_settings_value("reset_on_export", "0");
        } else {
            crate::modules::database::set_settings_value("reset_on_export", "1");
        }
    }

    fn reset_labels(&self) {
        self.akasha_blue_box_label.set_text("0");
        self.akasha_no_blue_box_label.set_text("0");
        self.akasha_coronation_ring_label.set_text("0");
        self.akasha_lineage_ring_label.set_text("0");
        self.akasha_intricacy_ring_label.set_text("0");
        self.akasha_trash_label.set_text("0");
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
        self.gohl_trash_label.set_text("0");
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
            "akasha_trash" => &self.akasha_trash_label,
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
            "gohl_trash" => &self.gohl_trash_label,
            _ => &self.pbhl_blue_box_label,
        };

        let mut total_chests = label.text().parse::<i32>().unwrap();

        if (label == &self.akasha_coronation_ring_label
            || label == &self.akasha_lineage_ring_label
            || label == &self.akasha_intricacy_ring_label
            || label == &self.akasha_gold_bar_label
            || label == &self.akasha_trash_label)
            && add
            && total_chests >= 0
        {
            self.add_blue_boxes(Raids::Akasha);
        }

        if (label == &self.akasha_coronation_ring_label
            || label == &self.akasha_lineage_ring_label
            || label == &self.akasha_intricacy_ring_label
            || label == &self.akasha_gold_bar_label
            || label == &self.akasha_trash_label)
            && !add
            && total_chests > 0
        {
            self.subtract_blue_boxes(Raids::Akasha);
        }

        if (label == &self.pbhl_coronation_ring_label
            || label == &self.pbhl_lineage_ring_label
            || label == &self.pbhl_intricacy_ring_label
            || label == &self.pbhl_gold_bar_label)
            && add
            && total_chests >= 0
        {
            self.add_blue_boxes(Raids::Pbhl);
        }

        if (label == &self.pbhl_coronation_ring_label
            || label == &self.pbhl_lineage_ring_label
            || label == &self.pbhl_intricacy_ring_label
            || label == &self.pbhl_gold_bar_label)
            && !add
            && total_chests > 0
        {
            self.subtract_blue_boxes(Raids::Pbhl);
        }

        if (label == &self.gohl_coronation_ring_label
            || label == &self.gohl_lineage_ring_label
            || label == &self.gohl_intricacy_ring_label
            || label == &self.gohl_gold_bar_label
            || label == &self.gohl_trash_label)
            && add
            && total_chests >= 0
        {
            self.add_blue_boxes(Raids::Gohl);
        }

        if (label == &self.gohl_coronation_ring_label
            || label == &self.gohl_lineage_ring_label
            || label == &self.gohl_intricacy_ring_label
            || label == &self.gohl_gold_bar_label
            || label == &self.gohl_trash_label)
            && !add
            && total_chests > 0
        {
            self.subtract_blue_boxes(Raids::Gohl);
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
            "akasha_trash" => &self.akasha_trash_label,
            "pbhl_coronation_rings" => &self.pbhl_coronation_ring_label,
            "pbhl_lineage_rings" => &self.pbhl_lineage_ring_label,
            "pbhl_intricacy_rings" => &self.pbhl_intricacy_ring_label,
            "pbhl_gold_bars" => &self.pbhl_gold_bar_label,
            "gohl_coronation_rings" => &self.gohl_coronation_ring_label,
            "gohl_lineage_rings" => &self.gohl_lineage_ring_label,
            "gohl_intricacy_rings" => &self.gohl_intricacy_ring_label,
            "gohl_gold_bars" => &self.gohl_gold_bar_label,
            "gohl_trash" => &self.gohl_trash_label,
            _ => return (),
        };

        let percentage = match field {
            "akasha_coronation_rings" => &self.akasha_coronation_ring_percentage,
            "akasha_lineage_rings" => &self.akasha_lineage_ring_percentage,
            "akasha_intricacy_rings" => &self.akasha_intricacy_ring_percentage,
            "akasha_gold_bars" => &self.akasha_gold_bar_percentage,
            "akasha_trash" => &self.akasha_trash_percentage,
            "pbhl_coronation_rings" => &self.pbhl_coronation_ring_percentage,
            "pbhl_lineage_rings" => &self.pbhl_lineage_ring_percentage,
            "pbhl_intricacy_rings" => &self.pbhl_intricacy_ring_percentage,
            "pbhl_gold_bars" => &self.pbhl_gold_bar_percentage,
            "gohl_coronation_rings" => &self.gohl_coronation_ring_percentage,
            "gohl_lineage_rings" => &self.gohl_lineage_ring_percentage,
            "gohl_intricacy_rings" => &self.gohl_intricacy_ring_percentage,
            "gohl_gold_bars" => &self.gohl_gold_bar_percentage,
            "gohl_trash" => &self.gohl_trash_percentage,
            _ => return (),
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
        if only_blues == "0" {
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
        let percentage_controls = vec![
            "akasha_coronation_rings",
            "akasha_lineage_rings",
            "akasha_intricacy_rings",
            "akasha_gold_bars",
            "akasha_trash",
            "pbhl_coronation_rings",
            "pbhl_lineage_rings",
            "pbhl_intricacy_rings",
            "pbhl_gold_bars",
            "gohl_coronation_rings",
            "gohl_lineage_rings",
            "gohl_intricacy_rings",
            "gohl_gold_bars",
            "gohl_trash",
        ];

        for percentage_control in percentage_controls {
            self.calculate_droprate(percentage_control);
        }
    }

    fn reset_droprates(&self) {
        let _ = &self.akasha_coronation_ring_percentage.set_text("");
        let _ = &self.akasha_lineage_ring_percentage.set_text("");
        let _ = &self.akasha_intricacy_ring_percentage.set_text("");
        let _ = &self.akasha_gold_bar_percentage.set_text("");
        let _ = &self.akasha_trash_percentage.set_text("");
        let _ = &self.pbhl_coronation_ring_percentage.set_text("");
        let _ = &self.pbhl_lineage_ring_percentage.set_text("");
        let _ = &self.pbhl_intricacy_ring_percentage.set_text("");
        let _ = &self.pbhl_gold_bar_percentage.set_text("");
        let _ = &self.gohl_coronation_ring_percentage.set_text("");
        let _ = &self.gohl_lineage_ring_percentage.set_text("");
        let _ = &self.gohl_intricacy_ring_percentage.set_text("");
        let _ = &self.gohl_gold_bar_percentage.set_text("");
        let _ = &self.gohl_trash_percentage.set_text("");
    }

    fn add_blue_boxes(&self, raid: Raids) {
        match raid {
            Raids::Akasha => self.change_value("akasha_blue_boxes", true),
            Raids::Pbhl => self.change_value("pbhl_blue_boxes", true),
            Raids::Gohl => self.change_value("gohl_blue_boxes", true),
            _ => return,
        }
    }

    fn subtract_blue_boxes(&self, raid: Raids) {
        match raid {
            Raids::Akasha => self.change_value("akasha_blue_boxes", false),
            Raids::Pbhl => self.change_value("pbhl_blue_boxes", false),
            Raids::Gohl => self.change_value("gohl_blue_boxes", false),
            _ => return,
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
                } else if btn == &self.akasha_trash_button {
                    self.change_value("akasha_trash", true);
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
                } else if btn == &self.gohl_trash_button {
                    self.change_value("gohl_trash", true);
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
                } else if btn == &self.akasha_trash_button {
                    self.change_value("akasha_trash", false);
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
                } else if btn == &self.gohl_trash_button {
                    self.change_value("gohl_trash", false);
                }
            }
            _ => unreachable!(),
        }
    }

    fn app_export_csv(&self) {
        let _ = &crate::modules::csv::export_csv();
        let reset_on_export = crate::modules::database::get_settings_value("reset_on_export");
        if reset_on_export == "1" {
            let _ = self.reset_labels();
            let _ = self.reset_droprates();
        }
    }

    fn kill_app(&self) {
        crate::modules::database::set_settings_value("last_active_tab", &self.tabs_container.selected_tab().to_string());
        nwg::stop_thread_dispatch();
    }
}
