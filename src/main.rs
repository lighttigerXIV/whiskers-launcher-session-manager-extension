use std::process::Command;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use simple_kl_rs::{
    actions::{ExtensionAction, ResultAction},
    extensions::get_parameters,
    extensions::Function::GetResults,
    extensions::{emit_results, Function::RunAction},
    paths::get_extension_icon,
    results::{IconWithTextResult, SimpleKLResult},
    settings::Settings,
};

fn main() {
    let parameters = get_parameters().unwrap();
    let function = parameters.function;
    let extension_id = "com-lighttigerxiv-session-manager";

    match function {
        GetResults => {
            let search_text = parameters.search_text.unwrap();
            let matcher = SkimMatcherV2::default();
            let mut results: Vec<SimpleKLResult> = Vec::new();

            if matcher.fuzzy_match("shutdown", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(extension_id, "@src/images/shutdown.svg").unwrap(),
                        "accent",
                        "Shutdown",
                        ResultAction::ExtensionAction(ExtensionAction::new(
                            extension_id,
                            "shutdown",
                        )),
                    ),
                ))
            }

            if matcher.fuzzy_match("reboot", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(extension_id, "@src/images/reboot.svg").unwrap(),
                        "accent",
                        "Reboot",
                        ResultAction::ExtensionAction(ExtensionAction::new(extension_id, "reboot")),
                    ),
                ))
            }

            if matcher.fuzzy_match("suspend", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(extension_id, "@src/images/zzz.svg").unwrap(),
                        "accent",
                        "Suspend",
                        ResultAction::ExtensionAction(ExtensionAction::new(
                            extension_id,
                            "suspend",
                        )),
                    ),
                ))
            }

            if matcher.fuzzy_match("hibernate", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(extension_id, "@src/images/zzz.svg").unwrap(),
                        "accent",
                        "Hibernate",
                        ResultAction::ExtensionAction(ExtensionAction::new(
                            extension_id,
                            "hibernate",
                        )),
                    ),
                ))
            }

            if matcher.fuzzy_match("logout", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(extension_id, "@src/images/logout.svg").unwrap(),
                        "accent",
                        "Logout",
                        ResultAction::ExtensionAction(ExtensionAction::new(extension_id, "logout")),
                    ),
                ))
            }

            emit_results(results);
        }
        RunAction => match parameters.action.unwrap().as_str() {
            "shutdown" => {
                Command::new("sh")
                    .arg("-c")
                    .arg("systemctl poweroff")
                    .output()
                    .expect("Error running poweroff command");
            }
            "reboot" => {
                Command::new("sh")
                    .arg("-c")
                    .arg("systemctl reboot")
                    .output()
                    .expect("Error running reboot command");
            }
            "suspend" => {
                Command::new("sh")
                    .arg("-c")
                    .arg("systemctl suspend")
                    .output()
                    .expect("Error running suspend command");
            }
            "hibernate" => {
                Command::new("sh")
                    .arg("-c")
                    .arg("systemctl hibernate")
                    .output()
                    .expect("Error running shutdown command");
            }
            "logout" => {
                let desktop_environment_setting =
                    Settings::get_extension_setting(extension_id, "desktop_environment").unwrap();

                match desktop_environment_setting.as_str() {
                    "gnome" => {
                        Command::new("sh")
                            .arg("-c")
                            .arg("gnome-session-quit --no-prompt")
                            .output()
                            .expect("Error running logout command");
                    }
                    "kde" => {
                        Command::new("sh")
                            .arg("-c")
                            .arg("qdbus org.kde.ksmserver /KSMServer logout 0 0 2")
                            .output()
                            .expect("Error running logout command");
                    }
                    "custom" => {
                        let custom_logout_setting =
                            Settings::get_extension_setting(extension_id, "custom_logout").unwrap();

                        Command::new("sh")
                            .arg("-c")
                            .arg(custom_logout_setting)
                            .output()
                            .expect("Error running logout command");
                    }
                    _ => {}
                };
            }
            _ => {}
        },
    }
}
