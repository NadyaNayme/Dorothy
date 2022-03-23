#![windows_subsystem = "windows"]

use native_windows_gui as nwg;
use nwg::NativeUi;

mod gui {
    pub mod gui;
}
mod modules {
    pub mod csv;
    pub mod database;
    pub mod utility;
}
use gui::gui::BarTracker;
use modules::database::create_new_log;

fn main() {
    create_new_log();
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = BarTracker::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
