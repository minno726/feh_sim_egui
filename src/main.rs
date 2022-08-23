#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "feh_sim_egui_canvas",
        web_options,
        Box::new(|cc| Box::new(feh_sim_egui::TemplateApp::new(cc))),
    )
    .expect("failed to start eframe");
}
