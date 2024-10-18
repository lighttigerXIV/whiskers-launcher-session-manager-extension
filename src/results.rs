use sniffer_rs::sniffer::Sniffer;
use whiskers_launcher_core::{
    features::extensions::{send_search_results, ExtensionRequest},
    results::{ResultAction, RunExtensionAction, SearchResult, SearchResults},
};

use crate::{assets::get_icon, ID};

pub fn show_results(request: ExtensionRequest) {
    let search_text = request.search_text.unwrap();
    let mut results: Vec<SearchResult> = vec![];
    let sniffer = Sniffer::new();

    if sniffer
        .clone()
        .matches("shutdown/shut down/power off/turn off", &search_text)
    {
        let action =
            ResultAction::new_run_extension_action(RunExtensionAction::new(ID, "shutdown"))
                .set_dangerous(true);

        let result = SearchResult::new("Shutdown", action)
            .set_icon(get_icon("shutdown"))
            .set_accent_icon_tint();

        results.push(result);
    }

    if sniffer.matches("restart/reboot", &search_text) {
        let action = ResultAction::new_run_extension_action(RunExtensionAction::new(ID, "restart"))
            .set_dangerous(true);

        let result = SearchResult::new("Restart", action)
            .set_icon(get_icon("restart"))
            .set_accent_icon_tint();

        results.push(result);
    }

    if sniffer.matches("suspend/sleep", &search_text) {
        let action = ResultAction::new_run_extension_action(RunExtensionAction::new(ID, "suspend"))
            .set_dangerous(true);

        let result = SearchResult::new("Suspend", action)
            .set_icon(get_icon("zzz"))
            .set_accent_icon_tint();

        results.push(result);
    }

    if sniffer.matches("hibernate/sleep", &search_text) {
        let action =
            ResultAction::new_run_extension_action(RunExtensionAction::new(ID, "hibernate"))
                .set_dangerous(true);

        let result = SearchResult::new("Hibernate", action)
            .set_icon(get_icon("zzz"))
            .set_accent_icon_tint();
        results.push(result);
    }

    if sniffer.matches("logout/log out", &search_text) {
        let action = ResultAction::new_run_extension_action(RunExtensionAction::new(ID, "logout"))
            .set_dangerous(true);

        let result = SearchResult::new("Logout", action)
            .set_icon(get_icon("logout"))
            .set_accent_icon_tint();

        results.push(result);
    }

    if sniffer.matches("lock", &search_text) {
        let action = ResultAction::new_run_extension_action(RunExtensionAction::new(ID, "lock"));
        let result = SearchResult::new("Lock", action)
            .set_icon(get_icon("lock"))
            .set_accent_icon_tint();

        results.push(result);
    }

    send_search_results(SearchResults::new_grid_results(results));
}
