// Description: file containing public functions 
// use log::error;
use std::path::PathBuf;
use rfd::FileDialog;
use crate::gui;

/// Open a file dialog to pick a file
pub fn open_fd() -> Option<PathBuf> {
    let f = FileDialog::new()
        .pick_file();
    return f
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
pub fn handle_task_selection(sel_task: &mut gui::Tasks, bool_vec: &mut Vec<bool>) {
    set_all_disabled(bool_vec);
    match sel_task {
        gui::Tasks::EnergyCalibration => {set_enabled(bool_vec, vec![3,5])},
        gui::Tasks::PlotSpectrum => {set_enabled(bool_vec, vec![3, 6])},
        gui::Tasks::CalculatePrimarySpectrum => {set_enabled(bool_vec, vec![0, 6])},
        gui::Tasks::CalculateFullSpectrum => {set_enabled(bool_vec, vec![0, 2, 6])},
        gui::Tasks::Calibrate => {set_enabled(bool_vec, vec![0, 1, 2, 5])},
        gui::Tasks::Quantify => {set_enabled(bool_vec, vec![0, 1, 3, 5, 6])},
        gui::Tasks::Evaluate => {set_enabled(bool_vec, vec![0, 1, 2, 4, 5])},
        gui::Tasks::Map => {set_enabled(bool_vec, vec![0, 1, 3, 4, 5])},
        gui::Tasks::CompareMeasuredCalculated => {set_enabled(bool_vec, vec![0, 2, 3, 6])},
        gui::Tasks::FitOneStandardWithPlot => {set_enabled(bool_vec, vec![0, 2, 5, 6])},
        gui::Tasks::BulkSumAndMaxValue => {set_enabled(bool_vec, vec![0, 1, 3, 5, 6])},
        gui::Tasks::OpticResponse => {set_enabled(bool_vec, vec![0, 2, 3, 5, 6])},
        gui::Tasks::None => {set_all_disabled(bool_vec)}
    }
}

        // [0,           1,          2,         3,             4,        5,                6,         7]
        // [config_file, calib_file, stds_file, spectrum_file, map_file, element_controls, plot_file, execute_button]