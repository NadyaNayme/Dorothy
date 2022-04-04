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
    #[nwg_control(size: (450, 240), position: (500, 500), title: "Dorothy", flags: "WINDOW", center:true)]
    #[nwg_events(OnWindowClose: [BarTracker::kill_app], OnInit: [BarTracker::init])]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(family: "Meiryo", size: 16)]
    font: nwg::Font,

    #[nwg_resource(family: "Meiryo UI", size: 14)]
    smallfont: nwg::Font,

    #[nwg_control(text: "&File", disabled: false, parent: BarTracker::window)]
    file_menu: nwg::Menu,

    #[nwg_control(text: "&Export", disabled: false, parent: BarTracker::file_menu)]
    #[nwg_events(OnMenuItemSelected: [BarTracker::app_export_csv])]
    export_menuitem: nwg::MenuItem,

    #[nwg_control(text: "&Settings", disabled: false, parent: BarTracker::window)]
    settings_menu: nwg::Menu,

    #[nwg_control(text: "&Always On Top", disabled: false, check: true, parent: BarTracker::settings_menu)]
    #[nwg_events(OnMenuItemSelected: [BarTracker::change_settings(SELF, CTRL)])]
    always_on_top_menuitem: nwg::MenuItem,

    #[nwg_control(text: "&Reset Counts on Export", disabled: false, check: true, parent: BarTracker::settings_menu)]
    #[nwg_events(OnMenuItemSelected: [BarTracker::change_settings(SELF, CTRL)])]
    reset_counts_menuitem: nwg::MenuItem,

    #[nwg_control(text: "&Calculate droprates by kills", disabled: false, check: false, parent: BarTracker::settings_menu)]
    #[nwg_events(OnMenuItemSelected: [BarTracker::change_settings(SELF, CTRL)])]
    calculate_drops_menuitem: nwg::MenuItem,

    #[nwg_control(text: "&Helpful Links", disabled: false, parent: BarTracker::window)]
    helpful_links_menu: nwg::Menu,

    #[nwg_control(text: "&Latest Dorothy Release (github.com)", disabled: false, parent: BarTracker::helpful_links_menu)]
    #[nwg_events(OnMenuItemSelected: [BarTracker::open_to_webpage(SELF, CTRL)])]
    dorothy_github_link: nwg::MenuItem,

    #[nwg_control(text: "GBF &Wiki (gbf.wiki)", disabled: false, parent: BarTracker::helpful_links_menu)]
    #[nwg_events(OnMenuItemSelected: [BarTracker::open_to_webpage(SELF, CTRL)])]
    gbf_wiki_link: nwg::MenuItem,

    #[nwg_control(text: "Online &Tools (granblue.party)", disabled: false, parent: BarTracker::helpful_links_menu)]
    #[nwg_events(OnMenuItemSelected: [BarTracker::open_to_webpage(SELF, CTRL)])]
    granblue_party_link: nwg::MenuItem,

    #[nwg_control(text: "&Raidfinder (gbf.life)", disabled: false, parent: BarTracker::helpful_links_menu)]
    #[nwg_events(OnMenuItemSelected: [BarTracker::open_to_webpage(SELF, CTRL)])]
    raidfinder_link: nwg::MenuItem,

    #[nwg_control(text: "/r/&Granblue_en (reddit.com)", disabled: false, parent: BarTracker::helpful_links_menu)]
    #[nwg_events(OnMenuItemSelected: [BarTracker::open_to_webpage(SELF, CTRL)])]
    subreddit_link: nwg::MenuItem,

    #[nwg_control(parent: Bartracker::window, size: (450, 240), font: Some(&data.smallfont))]
    #[nwg_events(TabsContainerChanged: [BarTracker::tab_changed])]
    tabs_container: nwg::TabsContainer,

    #[nwg_control(text: "Spark Calc", parent: BarTracker::tabs_container)]
    pulls_tab: nwg::Tab,

    #[nwg_control(text: "Akasha", parent: BarTracker::tabs_container)]
    akasha_tab: nwg::Tab,

    #[nwg_control(text: "PBHL", parent: BarTracker::tabs_container)]
    pbhl_tab: nwg::Tab,

    #[nwg_control(text: "GOHL", parent: BarTracker::tabs_container)]
    gohl_tab: nwg::Tab,

    #[nwg_layout(parent: BarTracker::tabs_container, margin: [10, 10, 10, 10], spacing: 2, min_size: [450, 200], max_size: [520, 200])]
    grid_layout: nwg::GridLayout,

    #[nwg_control(parent: BarTracker::pulls_tab, text: "Crystals", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 0, col_span: 2)]
    crystals_amount_label: nwg::Label,

    #[nwg_control(parent: BarTracker::pulls_tab, text: "10-Pull Tickets", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 1, col_span: 2)]
    ten_pull_tickets_amount_label: nwg::Label,

    #[nwg_control(parent: BarTracker::pulls_tab, text: "1-Pull Tickets", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 1, row: 2, col_span: 2)]
    single_pull_tickets_amount_label: nwg::Label,

    #[nwg_control(parent: BarTracker::pulls_tab, align: nwg::HTextAlign::Right, text: &crate::modules::database::get_db_value("crystals_amount"), flags: "VISIBLE|NUMBER", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 0, col_span: 1)]
    #[nwg_events(OnTextInput: [BarTracker::calculate_pulls_on_input(SELF, CTRL, EVT)])]
    crystals_amount: nwg::TextInput,

    #[nwg_control(parent: BarTracker::pulls_tab, align: nwg::HTextAlign::Right, text: &crate::modules::database::get_db_value("ten_pulls_amount"), flags: "VISIBLE|NUMBER", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 1, col_span: 1)]
    #[nwg_events(OnTextInput: [BarTracker::calculate_pulls_on_input(SELF, CTRL, EVT)])]
    ten_pull_tickets_amount: nwg::TextInput,

    #[nwg_control(parent: BarTracker::pulls_tab, align: nwg::HTextAlign::Right, text: &crate::modules::database::get_db_value("single_pulls_amount"), flags: "VISIBLE|NUMBER", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 2, col_span: 1)]
    #[nwg_events(OnTextInput: [BarTracker::calculate_pulls_on_input(SELF, CTRL, EVT)])]
    single_pull_tickets_amount: nwg::TextInput,

    #[nwg_control(parent: BarTracker::pulls_tab, h_align: nwg::HTextAlign::Center, text: "", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 0, row: 3, col_span: 3)]
    total_pulls_label: nwg::Label,

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
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_no_blue_box_button: nwg::Button,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_coronation_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_lineage_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_intricacy_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    akasha_gold_bars_button: nwg::Button,

    #[nwg_control(parent: BarTracker::akasha_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 6, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
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
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_no_blue_box_button: nwg::Button,

    #[nwg_control(parent: BarTracker::pbhl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_coronation_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::pbhl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_lineage_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::pbhl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    pbhl_intricacy_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::pbhl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
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
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_no_blue_box_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 2, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_coronation_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 3, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_lineage_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 4, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_intricacy_rings_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 5, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_gold_bars_button: nwg::Button,

    #[nwg_control(parent: BarTracker::gohl_tab, text: "+1", font: Some(&data.font))]
    #[nwg_layout_item(layout: grid_layout, col: 6, row: 3)]
    #[nwg_events(MousePressLeftDown: [BarTracker::button_actions(SELF, CTRL, EVT)], MousePressRightDown: [BarTracker::button_actions(SELF, CTRL, EVT)])]
    gohl_trash_button: nwg::Button,
}

impl BarTracker {
    fn init(&self) {
        let em = &self.embed;
        self.window.set_icon(em.icon_str("DOROTHY", None).as_ref());
        self.calculate_droprates();
        self.calculate_pulls();
        let last_window_position_x = crate::modules::database::get_settings_value("last_window_position_x").parse::<i32>().unwrap_or_default();
        let last_window_position_y = crate::modules::database::get_settings_value("last_window_position_y").parse::<i32>().unwrap_or_default();
        if last_window_position_x != 0 && last_window_position_y != 0 {
            self.window.set_position(last_window_position_x, last_window_position_y);
        }
        let last_active_tab = crate::modules::database::get_settings_value("last_active_tab").parse::<usize>().unwrap_or_default();
        self.tabs_container.set_selected_tab(last_active_tab);
        self.tab_changed();
        self.init_settings();
        self.window.set_visible(true);
    }

    fn init_settings(&self) {
        let app_rc = Rc::new(self);
        let topmost_setting = crate::modules::database::get_settings_value("always_on_top");
        let only_blues_setting = crate::modules::database::get_settings_value("only_blues");
        let export_reset_setting = crate::modules::database::get_settings_value("reset_on_export");
        if topmost_setting == String::from("1") {
            use winapi::um::winuser::SetWindowPos;
            use winapi::um::winuser::{HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE};
            self.always_on_top_menuitem.set_checked(true);
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
        } else {
            self.always_on_top_menuitem.set_checked(false);
        }
        if only_blues_setting == String::from("1") {
            self.calculate_drops_menuitem.set_checked(true);
        } else {
            self.calculate_drops_menuitem.set_checked(false);
        }
        if export_reset_setting == String::from("1") {
            self.reset_counts_menuitem.set_checked(true);
        } else {
            self.reset_counts_menuitem.set_checked(false);
        }
    }

    fn change_settings(&self, menuitem: &nwg::MenuItem) {
        let app_rc = Rc::new(self);
        if menuitem == &self.reset_counts_menuitem {
            BarTracker::set_export_reset(&self);
            menuitem.set_checked(!menuitem.checked());
        } else if menuitem == &self.always_on_top_menuitem {
            BarTracker::set_topmost(&app_rc);
            menuitem.set_checked(!menuitem.checked());
        } else if menuitem == &self.calculate_drops_menuitem {
            BarTracker::set_only_blues(&self);
            menuitem.set_checked(!menuitem.checked());
        }
    }

    fn set_topmost(app_rc: &Rc<&BarTracker>) {
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

    fn tab_changed(&self) {
        let sel_tab = self.tabs_container.selected_tab();
        if sel_tab == 0 {
            self.window.set_text("Dorothy - Pulls");
        } else if sel_tab == 1 {
            self.window.set_text("Dorothy - Akasha");
        } else if sel_tab == 2 {
            self.window.set_text("Dorothy - PBHL");
        } else if sel_tab == 3 {
            self.window.set_text("Dorothy - GOHL");
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
            _ => unreachable!(),
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
        let selected_tab = self.tabs_container.selected_tab();

        if total_akasha_blue_chests >= &0.0 && total_akasha_chests >= 1.0 && selected_tab == 1 as usize {
            percentage.set_text(&get_percentage(dropped_chests, total_akasha_chests));
        }
        if total_pbhl_blue_chests >= &0.0 && total_pbhl_chests >= 1.0 && selected_tab == 2 as usize {
            percentage.set_text(&get_percentage(dropped_chests, total_pbhl_chests));
        }
        if total_gohl_blue_chests >= &0.0 && total_gohl_chests >= 1.0 && selected_tab == 3 as usize {
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
        let selected_tab = self.tabs_container.selected_tab();

        for (i, control) in percentage_controls.iter().enumerate() {
            if i <= 4 && selected_tab == 1 as usize {
                self.calculate_droprate(control);
            } else if i >= 5 && i <= 8 && selected_tab == 2 as usize {
                self.calculate_droprate(control);
            } else if i >= 9 && selected_tab == 3 as usize {
                self.calculate_droprate(control);
            }
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

    fn calculate_pulls(&self) {
        let crystals_amount = Some(self.crystals_amount.text()).unwrap_or_default();
        let ten_pulls_amount = Some(self.ten_pull_tickets_amount.text()).unwrap_or_default();
        let single_pulls_amount = Some(self.single_pull_tickets_amount.text()).unwrap_or_default();
        crate::modules::database::set_db_value("crystals_amount", &crystals_amount);
        crate::modules::database::set_db_value("ten_pulls_amount", &ten_pulls_amount);
        crate::modules::database::set_db_value("single_pulls_amount", &single_pulls_amount);
        let total = BarTracker::calculate_crystal_pulls(crystals_amount.parse::<f32>().unwrap_or_default(), ten_pulls_amount.parse::<f32>().unwrap_or_default(), single_pulls_amount.parse::<f32>().unwrap_or_default());
        let spark_percentage = crate::modules::utility::get_percentage(total.parse::<f32>().unwrap(), 300.0);
        self.total_pulls_label.set_text(&("Total: ".to_owned() + &total + " | " + &spark_percentage + " of spark"));
    }

    fn calculate_pulls_on_input(&self, input: &nwg::TextInput, event: nwg::Event) {
        use nwg::Event as E;
        match event {
            E::OnTextInput => {
                let _ = &self.calculate_pulls();
            },
            _ => unreachable!(),
        }
    }

    fn calculate_crystal_pulls(crystals: f32, tenners: f32, singles: f32) -> String {
        let ten_pulls = ((crystals / 3000.0).floor()) * 10.0;
        let remaining_crystals_for_single_pulls = crystals % 3000.0;
        let single_pulls = (remaining_crystals_for_single_pulls / 300.0).floor();
        let total_pulls = (ten_pulls + single_pulls + (tenners * 10.0) + singles).to_string();
        String::from(total_pulls)
    }

    fn app_export_csv(&self) {
        let _ = &crate::modules::csv::export_csv();
        let reset_on_export = crate::modules::database::get_settings_value("reset_on_export");
        if reset_on_export == "1" {
            let _ = self.reset_labels();
            let _ = self.reset_droprates();
        }
    }

    fn open_to_webpage(&self, input: &nwg::MenuItem) {
        if input == &self.gbf_wiki_link {
            let url = "https://gbf.wiki/Main_Page";
            let _ = open::that(&url).unwrap();
        } else if input == &self.dorothy_github_link {
            let url = "https://github.com/NadyaNayme/Dorothy/releases/latest";
            let _ = open::that(&url).unwrap();
        } else if input == &self.subreddit_link {
            let url = "https://www.reddit.com/r/Granblue_en/";
            let _ = open::that(&url).unwrap();
        } else if input == &self.raidfinder_link {
            let url = "https://gbf.life/";
            let _ = open::that(&url).unwrap();
        } else if input == &self.granblue_party_link {
            let url = "https://www.granblue.party/";
            let _ = open::that(&url).unwrap();
        }  else {
            ();
        }
    }

    fn kill_app(&self) {
        crate::modules::database::set_settings_value("last_window_position_x", &self.window.position().0.to_string());
        crate::modules::database::set_settings_value("last_window_position_y", &self.window.position().1.to_string());
        crate::modules::database::set_settings_value("last_active_tab", &self.tabs_container.selected_tab().to_string());
        nwg::stop_thread_dispatch();
    }
}
