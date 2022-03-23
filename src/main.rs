#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwg::NativeUi;

mod gui;
mod modules;
use crate::gui::*;

fn main() {
    modules::database::create_new_log();
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = BarTracker::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
