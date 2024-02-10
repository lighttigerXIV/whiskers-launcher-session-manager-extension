//Add this to hide commands on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use whiskers_launcher_rs::api::extensions::{self, get_extension_context};

pub mod assets;
mod handle_actions;
mod handle_results;

const EXTENSION_ID: &str = "lighttigerxiv/session-manager";

fn main() {
    let context = get_extension_context().unwrap();

    match context.action {
        extensions::Action::GetResults => handle_results::handle_results(context.to_owned()),
        extensions::Action::RunAction => handle_actions::handle_actions(context.to_owned()),
    }
}
