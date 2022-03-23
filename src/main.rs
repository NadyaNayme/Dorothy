#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
use nwg::NativeUi;

mod modules;
mod gui;
use crate::gui::*;

fn main() {
    modules::database::create_new_log();
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = BarTracker::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
