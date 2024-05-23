use whiskers_launcher_rs::{
    action::{Action, ExtensionAction},
    api::extensions::{send_response, ExtensionRequest},
    result::{TextResult, WLResult},
    utils::fuzzy_matches,
};

use crate::{assets::get_icon, EXTENSION_ID};

pub fn handle_results(request: ExtensionRequest) {
    let search_text = request.search_text.unwrap();
    let mut results: Vec<WLResult> = vec![];

    if fuzzy_matches("shutdown/shut down/power off/turn off", &search_text) {
        let action = Action::new_extension(ExtensionAction::new(EXTENSION_ID, "shutdown"))
            .ask_confirmation(true);

        let result = WLResult::new_text(
            TextResult::new("Shutdown", action)
                .icon(get_icon("shutdown"))
                .tint("accent"),
        );
        results.push(result);
    }

    if fuzzy_matches("restart/reboot", &search_text) {
        let action = Action::new_extension(ExtensionAction::new(EXTENSION_ID, "restart"))
            .ask_confirmation(true);

        let result = WLResult::new_text(
            TextResult::new("Restart", action)
                .icon(get_icon("restart"))
                .tint("accent"),
        );
        results.push(result);
    }

    if fuzzy_matches("suspend/sleep", &search_text) {
        let action = Action::new_extension(ExtensionAction::new(EXTENSION_ID, "suspend"));

        let result = WLResult::new_text(
            TextResult::new("Suspend", action)
                .icon(get_icon("zzz"))
                .tint("accent"),
        );
        results.push(result);
    }

    if fuzzy_matches("hibernate/sleep", &search_text) {
        let action = Action::new_extension(ExtensionAction::new(EXTENSION_ID, "hibernate"))
            .ask_confirmation(true);

        let result = WLResult::new_text(
            TextResult::new("Hibernate", action)
                .icon(get_icon("zzz"))
                .tint("accent"),
        );
        results.push(result);
    }

    if fuzzy_matches("logout/log out", &search_text) {
        let action = Action::new_extension(ExtensionAction::new(EXTENSION_ID, "logout"))
            .ask_confirmation(true);

        let result = WLResult::new_text(
            TextResult::new("Logout", action)
                .icon(get_icon("logout"))
                .tint("accent"),
        );
        results.push(result);
    }

    if fuzzy_matches("lock", &search_text) {
        let action = Action::new_extension(ExtensionAction::new(EXTENSION_ID, "lock"));
        let result = WLResult::new_text(
            TextResult::new("Lock", action)
                .icon(get_icon("lock"))
                .tint("accent"),
        );
        results.push(result);
    }

    send_response(results);
}
