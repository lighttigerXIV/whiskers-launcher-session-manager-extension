//Add this to hide commands on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use whiskers_launcher_rs::api::extensions::{get_extension_request, ActionContext};


pub mod assets;
pub mod command;
mod handle_actions;
mod handle_results;

use handle_results::*;
use handle_actions::*;

const EXTENSION_ID: &str = "lighttigerxiv/session-manager";

fn main() {
    let request = get_extension_request();

    match request.action_context {
        ActionContext::ResultsRequest => handle_results(request.to_owned()),
        ActionContext::RunAction => handle_actions(request.to_owned()),
    }
}