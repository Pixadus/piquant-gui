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
    let native_options = eframe::NativeOptions::default();

    // 3. Start up app
    eframe::run_native(
        app_title,
        native_options,
        Box::new(move |cc| Box::new(
            gui::PiquantApp::new(cc)
        ))
    )
}