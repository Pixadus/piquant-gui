// Description: file containing public functions 
// use log::error;
use std::path::PathBuf;
use rfd::FileDialog;
use eframe::egui::Color32;

use crate::gui;

/// Open a file dialog to pick a file
pub fn open_fd() -> Option<PathBuf> {
    let f = FileDialog::new()
        .pick_file();
    f
}

/// Check to see if a file path is valid & exists
pub fn check_path(path: String) -> bool {
    let path = PathBuf::from(path);
    path.try_exists().unwrap()
}

/// Disable all elements in a boolean vec
pub fn set_all_disabled(bool_vec: &mut Vec<bool>){
    for b in bool_vec.iter_mut() {
        *b = false;
    }
}

/// Enable nth entry in a boolean vec
pub fn set_enabled(bool_vec: &mut Vec<bool>, indices: Vec<usize>) {
    for index in indices.iter() {
        bool_vec[*index] = true;
    }
}

/// Handle radio button pressing
pub fn handle_task_selection(sel_task: &mut gui::Tasks, bool_vec: &mut Vec<bool>, args: &mut Vec<String>) {
    // Reset enables and CLI arguments
    set_all_disabled(bool_vec);
    args.clear();

    match sel_task {
        gui::Tasks::EnergyCalibration => {set_enabled(bool_vec, vec![3,5]); args.push(String::from("ene"))},
        gui::Tasks::PlotSpectrum => {set_enabled(bool_vec, vec![3, 6]); args.push(String::from("plo"))},
        gui::Tasks::CalculatePrimarySpectrum => {set_enabled(bool_vec, vec![0, 6]); args.push(String::from("pri"))},
        gui::Tasks::CalculateFullSpectrum => {set_enabled(bool_vec, vec![0, 2, 6]); args.push(String::from("calc"))},
        gui::Tasks::Calibrate => {set_enabled(bool_vec, vec![0, 1, 2, 5]); args.push(String::from("cal"))},
        gui::Tasks::Quantify => {set_enabled(bool_vec, vec![0, 1, 3, 5, 6]); args.push(String::from("qua"))},
        gui::Tasks::Evaluate => {set_enabled(bool_vec, vec![0, 1, 2, 4, 5]); args.push(String::from("eva"))},
        gui::Tasks::Map => {set_enabled(bool_vec, vec![0, 1, 3, 4, 5]); args.push(String::from("map"))},
        gui::Tasks::CompareMeasuredCalculated => {set_enabled(bool_vec, vec![0, 2, 3, 6]); args.push(String::from("com"))},
        gui::Tasks::FitOneStandardWithPlot => {set_enabled(bool_vec, vec![0, 2, 5, 6]); args.push(String::from("fits"))}, // NOTE this doesn't show in PIQUANT help
        gui::Tasks::BulkSumAndMaxValue => {set_enabled(bool_vec, vec![0, 1, 3, 5, 6]); args.push(String::from("sum"))},
        gui::Tasks::OpticResponse => {set_enabled(bool_vec, vec![0, 2, 3, 5, 6]); args.push(String::from("opt"))},
        gui::Tasks::None => {set_all_disabled(bool_vec)}
    }
}

        // [0,           1,          2,         3,             4,        5,                6,         7]
        // [config_file, calib_file, stds_file, spectrum_file, map_file, element_controls, plot_file, execute_button]

/// Set textbox color depending if path is valid or not
pub fn set_valid_path_colors(text_vec: Vec<String>, color_vec: &mut Vec<Color32>, valid_vec: &mut Vec<bool>) {
    let mut index = 0;
    for text in text_vec.iter() {
        // Check if valid path
        if check_path(text.to_string()) {
            // Set background color to green to indicate valid
            color_vec[index] = Color32::from_rgb(22, 44, 30);
            valid_vec[index] = true;
        } else if ((index == 5) | (index == 6)) & text.is_empty() {
            // If optional arg (plot_file, log_file), it's valid to not supply anything - let's not set color though. 
            valid_vec[index] = true;
        } else {
            color_vec[index] = eframe::egui::Style::default().visuals.extreme_bg_color;
            valid_vec[index] = false;
        }
        index += 1;
    }
}

/// Update arguments based on selected task & supplied parameters
pub fn update_args(sel_task: &mut gui::Tasks, args: &mut Vec<String>, text_vec: Vec<String>) {
    // Args contains only the initial command now - update arg vec with relevant text boxes
    let mut indices: Vec<usize> = Vec::new();

    // Match selected task to associated text box indexes
    match sel_task {
        gui::Tasks::EnergyCalibration => {indices.extend([3,7])},
        gui::Tasks::PlotSpectrum => {indices.extend([3,6])},
        gui::Tasks::CalculatePrimarySpectrum => {indices.extend([0,6])},
        gui::Tasks::CalculateFullSpectrum => {indices.extend([0,2,6])},
        gui::Tasks::Calibrate => {indices.extend([0,1,2,7])},
        gui::Tasks::Quantify => {indices.extend([0,1,3,6,7])},
        gui::Tasks::Evaluate => {indices.extend([0,1,2,4,7])},
        gui::Tasks::Map => {indices.extend([0,1,2,3,4,7])},
        gui::Tasks::CompareMeasuredCalculated => {indices.extend([0,2,3,6])},
        gui::Tasks::FitOneStandardWithPlot => {indices.extend([0,2,6,7])}, // NOTE this doesn't show in PIQUANT help
        gui::Tasks::BulkSumAndMaxValue => {indices.extend([0,1,3,6,7])},
        gui::Tasks::OpticResponse => {indices.extend([0,2,3,6,7])},
        gui::Tasks::None => {}
    }

    // Push associated text values to arguments
    for index in indices.iter() {
        args.push(text_vec[*index].clone())
    }
}

/// Validate if paths are valid for selected indices; primarily used in analyze() command.
fn check_valid(valid_vec: &mut Vec<bool>, indices: Vec<usize>) -> bool {
    let mut all_valid: bool = true;
    for index in indices.iter() {
        if ! valid_vec[*index] {
            all_valid = false;
        }
    }
    return all_valid;
}

/// Determine if we should enable the execute button
pub fn check_ready_to_execute(valid_vec: &mut Vec<bool>, sel_task: &mut gui::Tasks, enable_vec: &mut Vec<bool>) {
    let mut indices: Vec<usize> = Vec::new();
    enable_vec[7] = false;

    // Match selected task to associated text box indexes
    match sel_task {
        gui::Tasks::EnergyCalibration => {indices.extend([3,5])},
        gui::Tasks::PlotSpectrum => {indices.extend([3,6])},
        gui::Tasks::CalculatePrimarySpectrum => {indices.extend([0,6])},
        gui::Tasks::CalculateFullSpectrum => {indices.extend([0,2,6])},
        gui::Tasks::Calibrate => {indices.extend([0,1,2,5])},
        gui::Tasks::Quantify => {indices.extend([0,1,3,5,6])},
        gui::Tasks::Evaluate => {indices.extend([0,1,2,4,5])},
        gui::Tasks::Map => {indices.extend([0,1,2,3,4,5])},
        gui::Tasks::CompareMeasuredCalculated => {indices.extend([0,2,3,6])},
        gui::Tasks::FitOneStandardWithPlot => {indices.extend([0,2,5,6])}, // NOTE this doesn't show in PIQUANT help
        gui::Tasks::BulkSumAndMaxValue => {indices.extend([0,1,3,5,6])},
        gui::Tasks::OpticResponse => {indices.extend([0,2,3,5,6])},
        gui::Tasks::None => {}
    }

    enable_vec[7] = check_valid(valid_vec, indices);
}