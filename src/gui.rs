// Description: file used to set up and manage the program window. 

use std::path::PathBuf;
use rfd::FileDialog;
use eframe::egui;

use crate::functions::get_file;

pub struct PiquantApp {
    // Vector containing valid files in directory
    images: Option<Vec<PathBuf>>,

    // Current index of images
    im_index: Option<usize>,
}

/// Set up the app with initial values
impl PiquantApp {
    /// Initial application setup
    pub fn new(_cc: &eframe::CreationContext<'_>, initial_image: Option<PathBuf>) -> Self {
        // If some initial image, return some Vec.
        match initial_image {
            Some(i) => Self {
                images: Some(vec![i]),
                im_index: Some(0)
            },
            None => Self {
                images: None,
                im_index: None
            }
        }
    }
}

impl eframe::App for PiquantApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {images, im_index } = self;

        // Menu panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // This is where we store our menu bar
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // Open file dialogue
                        let opened_file = FileDialog::new()
                            .set_directory(match std::env::current_dir() {
                                Ok(d) => d,
                                Err(_e) => PathBuf::from("/")
                            })
                            .pick_file();
                        match opened_file {
                            Some(f) => {
                                // Check extension - change FM to only look for these
                                match get_file(f) {
                                    Some(f) => {},
                                    None => {}
                                }
                                // *images = Some(vec!(f))
                            },
                            None => {}
                        }
                    };
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    };
                });
            });
        });
        println!("{:?}", images);
    }
}