//Add this to hide commands on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


pub mod assets;
pub mod command;
mod actions;
mod results;

use results::*;
use actions::*;
use whiskers_launcher_core::features::core::extensions::get_extension_request;

const ID: &str = "lighttigerxiv/session-manager";

fn main() {
    let request = get_extension_request();

    match request.request_type {
        whiskers_launcher_core::features::extensions::ExtensionRequestType::GetResults => {
            show_results(request.to_owned())
        },
        whiskers_launcher_core::features::extensions::ExtensionRequestType::RunCommand => {
            run_command(request.to_owned())
        },
    }
}