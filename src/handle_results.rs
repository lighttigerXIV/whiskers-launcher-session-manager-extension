use whiskers_launcher_rs::api::extensions::Context;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use whiskers_launcher_rs::{
    actions,
    api::extensions::send_extension_results,
    results::{self, WhiskersResult},
};

use crate::{assets::get_icon, EXTENSION_ID};

pub fn handle_results(context: Context) {
    let search_text = context.search_text.unwrap();
    let mut results: Vec<WhiskersResult> = vec![];
    let matcher = SkimMatcherV2::default();

    if matcher
        .fuzzy_match("shutdown/power off/turn off", &search_text)
        .is_some()
    {
        results.push(WhiskersResult::Text(
            results::Text::new(
                "Shut Down",
                actions::Action::Extension(actions::Extension::new(EXTENSION_ID, "shut_down")),
            )
            .icon(get_icon("shutdown.svg"))
            .tint_icon(true),
        ));
    }

    if matcher
        .fuzzy_match("reboot/restart", &search_text)
        .is_some()
    {
        results.push(WhiskersResult::Text(
            results::Text::new(
                "Reboot",
                actions::Action::Extension(actions::Extension::new(EXTENSION_ID, "reboot")),
            )
            .icon(get_icon("reboot.svg"))
            .tint_icon(true),
        ));
    }

    if matcher.fuzzy_match("suspend", &search_text).is_some() {
        results.push(WhiskersResult::Text(
            results::Text::new(
                "Suspend",
                actions::Action::Extension(actions::Extension::new(EXTENSION_ID, "suspend")),
            )
            .icon(get_icon("zzz.svg"))
            .tint_icon(true),
        ));
    }

    if matcher.fuzzy_match("hibernate", &search_text).is_some() {
        results.push(WhiskersResult::Text(
            results::Text::new(
                "Hibernate",
                actions::Action::Extension(actions::Extension::new(EXTENSION_ID, "hibernate")),
            )
            .icon(get_icon("zzz.svg"))
            .tint_icon(true),
        ));
    }

    if matcher.fuzzy_match("logout", &search_text).is_some() {
        results.push(WhiskersResult::Text(
            results::Text::new(
                "Logout",
                actions::Action::Extension(actions::Extension::new(EXTENSION_ID, "logout")),
            )
            .icon(get_icon("logout.svg"))
            .tint_icon(true),
        ));
    }

    if matcher.fuzzy_match("lock", &search_text).is_some() {
        results.push(WhiskersResult::Text(
            results::Text::new(
                "Lock",
                actions::Action::Extension(actions::Extension::new(EXTENSION_ID, "lock")),
            )
            .icon(get_icon("lock.svg"))
            .tint_icon(true),
        ));
    }

    send_extension_results(results);
}
