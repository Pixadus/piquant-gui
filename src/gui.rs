// Description: file used to set up and manage the program window. 
// An excellent guide on some egui setup methods is available at https://egui.info/examples/

use eframe::egui;

// Possible task options
#[derive(Debug, PartialEq)]
enum Tasks {
    Energy_Calibration,
    Plot_Spectrum,
    Calculate_Primary_Spectrum,
    Calculate_Full_Spectrum,
    Compare_Measured_to_Calculated,
    Optic_Response,
    Calibrate,
    Evaluate,
    Fit_one_standard_with_plot,
    Quantify,
    Bulk_sum_and_max_value,
    Map,
    None
}

pub struct PiquantApp {
    task_sel: Tasks,
    config_file: String
}

/// Set up the app with initial values
impl PiquantApp {
    // Initial application setup
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Provide initial values
        Self{
            task_sel: Tasks::None,
            config_file: String::new()
        }
    }
}

impl eframe::App for PiquantApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { 
            task_sel,
            config_file
        } = self;

        // Create a central panel to hold our widgets in the window
        egui::CentralPanel::default().show(ctx, |ui| {

            // Task selection section
            ui.heading("Task selection");

            // Grid to contain task list (see Tasks enum)
            egui::Grid::new("task_selection")
                .striped(true)
                .spacing([25.0, 10.0])
                .show(ui, |ui| {
                    ui.radio_value(task_sel, Tasks::Energy_Calibration, "Energy Calibration");
                    ui.radio_value(task_sel, Tasks::Plot_Spectrum, "Plot Spectrum");
                    ui.radio_value(task_sel, Tasks::Calculate_Primary_Spectrum, "Calculate Primary Spectrum");
                    ui.radio_value(task_sel, Tasks::Calculate_Full_Spectrum, "Calculate Full Spectrum");
                    ui.end_row();
                
                    ui.radio_value(task_sel, Tasks::Compare_Measured_to_Calculated, "Compare Measured to Calculated");
                    ui.radio_value(task_sel, Tasks::Optic_Response, "Optic Response");
                    ui.radio_value(task_sel, Tasks::Calibrate, "Calibrate");
                    ui.radio_value(task_sel, Tasks::Evaluate, "Evaluate");
                    ui.end_row();
                
                    ui.radio_value(task_sel, Tasks::Fit_one_standard_with_plot, "Fit one standard with plot");
                    ui.radio_value(task_sel, Tasks::Quantify, "Quantify");
                    ui.radio_value(task_sel, Tasks::Bulk_sum_and_max_value, "Bulk sum and max value");
                    ui.radio_value(task_sel, Tasks::Map, "Map");
                    ui.end_row();
                }
            );

            // Separator
            ui.separator();

            // Configuration section
            ui.heading("Configuration");

            egui::Grid::new("configuration")
                .spacing([50.0, 12.5])
                .num_columns(2)
                .show(ui, |ui| {
                    // Config file
                    ui.add_space(125.0); // Add some initial margin 
                    ui.add(egui::Label::new("Configuration file"));
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(config_file).hint_text("path to config file"));
                        ui.button("Browse");
                    });
                    ui.end_row();

                    // Calibration file
                    ui.add_space(125.0);
                    ui.add(egui::Label::new("Calibration file"));
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(config_file).hint_text("path to calibration file"));
                        ui.button("Browse");
                    });
                    ui.end_row();

                    // Standards file
                    ui.add_space(125.0);
                    ui.add(egui::Label::new("Standards input file"));
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(config_file).hint_text("path to standards input file"));
                        ui.button("Browse");
                    });
                    ui.end_row();

                    // Spectrum file
                    ui.add_space(125.0);
                    ui.add(egui::Label::new("Spectrum file"));
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(config_file).hint_text("path to spectrum file"));
                        ui.button("Browse");
                    });
                    ui.end_row();

                    // Map file
                    ui.add_space(125.0);
                    ui.add(egui::Label::new("Map file"));
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(config_file).hint_text("path to map file (optional?)"));
                        ui.button("Browse");
                    });
                    ui.end_row();

                    // Element fit controls
                    ui.add_space(125.0);
                    ui.add(egui::Label::new("Element fit controls"));
                    ui.add(egui::TextEdit::singleline(config_file).hint_text("FE_[KLMN] [IFX]").desired_width(340.0));
                    ui.end_row();
                }
            );

            // Separator
            ui.separator();

            // Optional arguments section
            ui.heading("Optional arguments");
            egui::Grid::new("optional_arguments")
                .spacing([50.0, 15.0])
                .num_columns(2)
                .show(ui, |ui| {
                    // Plot file
                    ui.add_space(125.0);
                    ui.add(egui::Label::new("Plot file"));
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(config_file).hint_text("path to plot file (optional)"));
                        ui.button("Browse");
                    });
                    ui.end_row();
                    
                    // Log file
                    ui.add_space(125.0);
                    ui.add(egui::Label::new("Log file (appends)"));
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(config_file).hint_text("path to log file (optional)"));
                        ui.button("Browse");
                    });
                    ui.end_row();

                    // Extra CLI arguments
                    ui.add_space(125.0);
                    ui.add(egui::Label::new("CLI arguments"));
                    ui.add(egui::TextEdit::singleline(config_file).hint_text("additional CLI arguments").desired_width(340.0));
                    ui.end_row();
                }
            );

            // Separator
            ui.separator();

            // Results section
            ui.add_enabled(false,
                egui::TextEdit::multiline(config_file)
                    .hint_text("output").desired_width(f32::INFINITY)
                    .desired_rows(8)
                );

            // "Execute" button
            ui.vertical_centered(|ui| {
                ui.add(egui::Button::new("Execute").min_size(egui::Vec2::new(775.0, 20.0)));
            });
        });
    }
}