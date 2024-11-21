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
    option_sel: Tasks
}

/// Set up the app with initial values
impl PiquantApp {
    // Initial application setup
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Provide initial values
        Self{
            option_sel: Tasks::None
        }
    }
}

impl eframe::App for PiquantApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { 
            option_sel 
        } = self;

        // Create a central panel to hold our widgets in the window
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical_centered(|ui| {
                ui.heading("Task Selection");
            });
            // Grid to contain task list (see Tasks enum)
            egui::Grid::new("tasks_top_row")
                .striped(true)
                .spacing(egui::Vec2::new(25.0, 10.0))
                .show(ui, |ui| {
                    ui.radio_value(option_sel, Tasks::Energy_Calibration, "Energy Calibration");
                    ui.radio_value(option_sel, Tasks::Plot_Spectrum, "Plot Spectrum");
                    ui.radio_value(option_sel, Tasks::Calculate_Primary_Spectrum, "Calculate Primary Spectrum");
                    ui.radio_value(option_sel, Tasks::Calculate_Full_Spectrum, "Calculate Full Spectrum");
                    ui.end_row();
                
                    ui.radio_value(option_sel, Tasks::Compare_Measured_to_Calculated, "Compare Measured to Calculated");
                    ui.radio_value(option_sel, Tasks::Optic_Response, "Optic Response");
                    ui.radio_value(option_sel, Tasks::Calibrate, "Calibrate");
                    ui.radio_value(option_sel, Tasks::Evaluate, "Evaluate");
                    ui.end_row();
                
                    ui.radio_value(option_sel, Tasks::Fit_one_standard_with_plot, "Fit one standard with plot");
                    ui.radio_value(option_sel, Tasks::Quantify, "Quantify");
                    ui.radio_value(option_sel, Tasks::Bulk_sum_and_max_value, "Bulk sum and max value");
                    ui.radio_value(option_sel, Tasks::Map, "Map");
                    ui.end_row();
                }
            );

            // Separator
            ui.separator();

            // Configuration section
            ui.vertical_centered(|ui| {
                ui.heading("Configuration");
            });
        });
    }
}