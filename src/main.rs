// Name: Neptune
// Author: Parker Lamb
// Description: Lightweight and speedy image-viewing application. 

mod functions;
mod input;
mod gui;

fn main() -> eframe::Result<()> {
    // Initialize logging system
    pretty_env_logger::init();
    
    // 1. Get application CLI arguments
    let arguments = input::get_args();

    // 2. Set up GUI options
    let app_title = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_min_inner_size(eframe::egui::vec2(800.0, 660.0))
            .with_resizable(false),
            ..Default::default()
    };

    // 3. Start up app
    eframe::run_native(
        app_title,
        native_options,
        Box::new(move |cc| Ok(Box::new(gui::PiquantApp::new(cc))))
    )
}