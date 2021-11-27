#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod gui;
mod packetsniffer;

#[cfg(not(target_arch = "wasm32"))]
fn main() 
{

    let gui = gui::OurApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(gui), native_options);

}
