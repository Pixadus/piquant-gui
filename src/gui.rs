// Description: file used to set up and manage the program window. 
// An excellent guide on some egui setup methods is available at https://egui.info/examples/

use log::info;
use eframe::egui::{self, TextBuffer};
use egui::Color32;
use std::env;
use std::process::Command;
use crate::functions;

// Possible task options
#[derive(Debug, PartialEq)]
pub enum Tasks {
    EnergyCalibration,
    PlotSpectrum,
    CalculatePrimarySpectrum,
    CalculateFullSpectrum,
    CompareMeasuredCalculated,
    OpticResponse,
    Calibrate,
    Evaluate,
    FitOneStandardWithPlot,
    Quantify,
    BulkSumAndMaxValue,
    Map,
    None
}

pub struct PiquantApp {
    task_sel: Tasks,
    config_file: String,
    calib_file: String,
    standards_file: String,
    spectrum_file: String,
    map_file: String,
    plot_file: String,
    log_file: String,
    element_controls: String,
    cli_args: String,
    output_text: String,
    enable_vec: Vec<bool>,
    color_vec: Vec<Color32>,
    valid_vec: Vec<bool>,
    args: Vec<String>
}

/// Set up the app with initial values
impl PiquantApp {
    // Initial application setup
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Get default style - may be dark or light depending on user theme
        let default_bg = egui::Style::default().visuals.extreme_bg_color;

        // Provide initial values
        Self{
            task_sel: Tasks::None,
            config_file: String::new(),
            calib_file: String::new(),
            standards_file: String::new(),
            spectrum_file: String::new(),
            map_file: String::new(),
            plot_file: String::new(),
            log_file: String::new(),
            element_controls: String::new(),
            cli_args: String::new(),
            output_text: String::new(),
            enable_vec: vec![false, false, false, false, false, false, false, false],
            color_vec: vec![default_bg, default_bg, default_bg, default_bg, default_bg, default_bg, default_bg, default_bg, default_bg],
            valid_vec: vec![false, false, false, false, false, false, false, false, false],
            args: Vec::new()
        }
        // Regarding enable vec:
        // [0,           1,          2,         3,             4,        5,                6,         7]
        // [config_file, calib_file, stds_file, spectrum_file, map_file, element_controls, plot_file, execute_button]

        // Color vec:
        // [0,           1,          2,         3,             4,        5,         6]
        // [config_file, calib_file, stds_file, spectrum_file, map_file, plot_file, log_file]

        // Text vec:
        // [0,           1,          2,         3,             4,        5,         6,        7,                8]
        // [config_file, calib_file, stds_file, spectrum_file, map_file, plot_file, log_file, element_controls, cli_args]
        // plot_file & log_file are optional (5, 6), ditto cli_args
    }
}
impl eframe::App for PiquantApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { 
            task_sel,
            config_file,
            calib_file,
            standards_file,
            spectrum_file,
            map_file,
            plot_file,
            log_file,
            element_controls,
            cli_args,
            output_text,
            enable_vec,
            color_vec,
            valid_vec,
            args
        } = self;

        // -------- Functions to run per app update

        // Update text_vec to values of each file field
        let text_vec: Vec<String> = vec![config_file.clone(), calib_file.clone(), standards_file.clone(), spectrum_file.clone(), map_file.clone(), plot_file.clone(), log_file.clone(), element_controls.clone(), cli_args.clone()];

        // Update enables to match selected tasks
        functions::handle_task_selection(task_sel, enable_vec, args);

        // Check each path for validity. Ask Tim re: colors. 
        functions::set_valid_path_colors(text_vec.clone(), color_vec, valid_vec);

        // Update args vec based on current selection and parameters
        functions::update_args(task_sel, args, text_vec);

        // Check if execute button is ready to go
        functions::check_ready_to_execute(valid_vec, task_sel, enable_vec);

        // ---------- UI building section 

        // Create a central panel to hold our widgets in the window
        egui::CentralPanel::default().show(ctx, |ui| {

            // Task selection section
            ui.heading("Task selection");
            ui.add_space(10.0);

            // Grid to contain task list (see Tasks enum)
            egui::Grid::new("task_selection")
                .striped(true)
                .spacing([25.0, 10.0])
                .show(ui, |ui| {
                    ui.radio_value(task_sel, Tasks::EnergyCalibration, "Energy Calibration");
                    ui.radio_value(task_sel, Tasks::PlotSpectrum, "Plot Spectrum");
                    ui.radio_value(task_sel, Tasks::CalculatePrimarySpectrum, "Calculate Primary Spectrum");
                    ui.radio_value(task_sel, Tasks::CalculateFullSpectrum, "Calculate Full Spectrum");
                    ui.end_row();
                
                    ui.radio_value(task_sel, Tasks::CompareMeasuredCalculated, "Compare Measured to Calculated");
                    ui.radio_value(task_sel, Tasks::OpticResponse, "Optic Response");
                    ui.radio_value(task_sel, Tasks::Calibrate, "Calibrate");
                    ui.radio_value(task_sel, Tasks::Evaluate, "Evaluate");
                    ui.end_row();
                
                    ui.radio_value(task_sel, Tasks::FitOneStandardWithPlot, "Fit one standard with plot");
                    ui.radio_value(task_sel, Tasks::Quantify, "Quantify");
                    ui.radio_value(task_sel, Tasks::BulkSumAndMaxValue, "Bulk sum and max value");
                    ui.radio_value(task_sel, Tasks::Map, "Map");
                    ui.end_row();
                }
            );
            info!("Created grid");

            // Separator
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            // Configuration section
            ui.heading("Configuration");
            ui.add_space(10.0);
            
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.add_space(125.0);
                egui::Grid::new("configuration")
                    .spacing([50.0, 12.5])
                    .num_columns(2)
                    .show(ui, |ui| {
                        // Config file
                        ui.add(egui::Label::new("Configuration file"));
                        ui.horizontal(|ui| {
                            ui.add_enabled(enable_vec[0], egui::TextEdit::singleline(config_file)
                                .hint_text("path to configuration file")
                                .background_color(color_vec[0]));
                            if ui.add_enabled(enable_vec[0], egui::Button::new("Browse")).clicked() {
                                // Open file dialog to look for relevant config files
                                let f = functions::open_fd();
                                if let Some(path) = f {
                                    *config_file =  path.into_os_string().into_string().unwrap();
                                }
                            };
                        });
                        ui.end_row();

                        // Calibration file
                        ui.add( egui::Label::new("Calibration file"));
                        ui.horizontal(|ui| {
                            ui.add_enabled(enable_vec[1], egui::TextEdit::singleline(calib_file)
                                .hint_text("path to calibration file")
                                .background_color(color_vec[1]));
                            if ui.add_enabled(enable_vec[1], egui::Button::new("Browse")).clicked() {
                                // Open file dialog to look for relevant config files
                                let f = functions::open_fd();
                                if let Some(path) = f {
                                    *calib_file =  path.into_os_string().into_string().unwrap();
                                }
                            };
                        });
                        ui.end_row();

                        // Standards file
                        ui.add(egui::Label::new("Standards input file"));
                        ui.horizontal(|ui| {
                            ui.add_enabled(enable_vec[2], egui::TextEdit::singleline(standards_file)
                                .hint_text("path to standards input file")
                                .background_color(color_vec[2]));
                            if ui.add_enabled(enable_vec[2], egui::Button::new("Browse")).clicked() {
                                // Open file dialog to look for relevant config files
                                let f = functions::open_fd();
                                if let Some(path) = f {
                                    *standards_file =  path.into_os_string().into_string().unwrap();
                                }
                            };
                        });
                        ui.end_row();

                        // Spectrum file
                        ui.add(egui::Label::new("Spectrum file"));
                        ui.horizontal(|ui| {
                            ui.add_enabled(enable_vec[3], egui::TextEdit::singleline(spectrum_file)
                                .hint_text("path to spectrum file")
                                .background_color(color_vec[3]));
                            if ui.add_enabled(enable_vec[3], egui::Button::new("Browse")).clicked() {
                                // Open file dialog to look for relevant config files
                                let f = functions::open_fd();
                                if let Some(path) = f {
                                    *spectrum_file =  path.into_os_string().into_string().unwrap();
                                }
                            };
                        });
                        ui.end_row();

                        // Map file
                        ui.add(egui::Label::new("Map file"));
                        ui.horizontal(|ui| {
                            ui.add_enabled(enable_vec[4], egui::TextEdit::singleline(map_file)
                                .hint_text("path to map file (optional?)")
                                .background_color(color_vec[4]));
                            if ui.add_enabled(enable_vec[4], egui::Button::new("Browse")).clicked() {
                                // Open file dialog to look for relevant config files
                                let f = functions::open_fd();
                                if let Some(path) = f {
                                    *map_file =  path.into_os_string().into_string().unwrap();
                                }
                            };
                        });
                        ui.end_row();

                        // Element fit controls
                        ui.add(egui::Label::new("Element fit controls"));
                        ui.add_enabled(enable_vec[5], egui::TextEdit::singleline(element_controls).hint_text("FE_[KLMN] [IFX]").desired_width(340.0));
                        ui.end_row();
                    }
                );
            });

            // Separator
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            // Optional arguments section
            ui.heading("Optional arguments");
            ui.add_space(10.0);

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.add_space(125.0);

                egui::Grid::new("optional_arguments")
                    .spacing([50.0, 15.0])
                    .num_columns(2)
                    .show(ui, |ui| {
                        // Plot file
                        ui.add(egui::Label::new("Plot file"));
                        ui.horizontal(|ui| {
                            ui.add_enabled(enable_vec[6], egui::TextEdit::singleline(plot_file)
                                .hint_text("path to plot file (optional)")
                                .background_color(color_vec[5]));
                            if ui.add_enabled(enable_vec[6], egui::Button::new("Browse")).clicked() {
                                // Open file dialog to look for relevant config files
                                let f = functions::open_fd();
                                if let Some(path) = f {
                                    *plot_file =  path.into_os_string().into_string().unwrap();
                                }
                            };
                        });
                        ui.end_row();
                        
                        // Log file
                        ui.add(egui::Label::new("Log file (appends)"));
                        ui.horizontal(|ui| {
                            ui.add(egui::TextEdit::singleline(log_file)
                                .hint_text("path to log file (optional)")
                                .background_color(color_vec[6]));
                            if ui.button("Browse").clicked() {
                                // Open file dialog to look for relevant config files
                                let f = functions::open_fd();
                                if let Some(path) = f {
                                    *log_file =  path.into_os_string().into_string().unwrap();
                                }
                            };
                        });
                        ui.end_row();

                        // Extra CLI arguments
                        ui.add(egui::Label::new("CLI arguments"));
                        ui.add(egui::TextEdit::singleline(cli_args).hint_text("additional CLI arguments").desired_width(340.0));
                        ui.end_row();
                    }
                );
            });

            // Separator
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            // Results multi-line textbox section
            egui::ScrollArea::vertical()
                .auto_shrink(true)
                .max_height(130.0)
                .show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                        |ui| {
                            ui.add_enabled(false,
                                egui::TextEdit::multiline(output_text)
                                    .hint_text("output").desired_width(f32::INFINITY)
                                    .desired_rows(8)
                            );
                        },
                    );
                });

            // "Execute" button
            ui.vertical_centered(|ui| {
                if ui.add_enabled(enable_vec[7], egui::Button::new("Execute").min_size(egui::Vec2::new(775.0, 20.0))).clicked() {
                    // Get CWD of Rust piquant executable, assuming the path is accessible.
                    let cwe = env::current_exe().unwrap();
                    let cwd = cwe.parent().unwrap();

                    // Relative to the executable path, the PIQUANT CLI exe should be (unless otherwise specified) in ../../lib/cli/bin/
                    let piquant_exe_dir = format!("{}/../../lib/cli/bin/", cwd.to_str().unwrap());
                    let piquant_exe;

                    // Clear output text
                    output_text.clear();

                    // Send arguments to PIQUANT and execute based on platform.
                    let output = if cfg!(target_os = "windows") {
                        piquant_exe = "PIQUANT.exe";
                        Command::new([piquant_exe_dir.as_str(), piquant_exe].concat())
                            .args(args.clone())
                            .output()
                            .expect("failed to execute process")
                    } else if cfg!(target_os = "macos") {
                        piquant_exe = "PIQUANT";
                        Command::new([piquant_exe_dir.as_str(), piquant_exe].concat())
                            .args(args.clone())
                            .output()
                            .expect("failed to execute process")
                    } else {
                        piquant_exe = "PIQUANT";
                        // (Likely) on *nix
                        Command::new([piquant_exe_dir.as_str(), piquant_exe].concat())
                            .args(args.clone())
                            .output()
                            .expect("failed to execute process")
                    };
                    
                    // Write to output. 
                    // output_text.push_str(format!("Executable path: {}\n", [piquant_exe_dir.as_str(), piquant_exe].concat()).as_str());
                    output_text.push_str(format!("Arguments: {:?}\n", args).as_str());
                    // output_text.push_str("--------------------------- Starting execution -------------------------------\n");
                    let stdout =  String::from_utf8(output.stdout).unwrap();
                    // output_text();
                    output_text.push_str(&stdout);
                };
            });

        });
    }
}