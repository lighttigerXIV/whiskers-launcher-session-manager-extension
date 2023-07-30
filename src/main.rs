use std::process::Command;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use simple_kl_rs::{
    actions::{ExtensionAction, ResultAction},
    extensions::get_parameters,
    extensions::Function::GetResults,
    extensions::{return_results, Function::RunAction},
    paths::get_extension_icon,
    paths::get_extension_path,
    results::{IconWithTextResult, SimpleKLResult},
    settings::Settings,
};

fn main() {
    let parameters = get_parameters();
    let function = parameters.function;
    let extension_id = "com-lighttigerxiv-session-manager".to_string();

    match function {
        GetResults => {
            let search_text = parameters.search_text.unwrap();
            let matcher = SkimMatcherV2::default();
            let mut results: Vec<SimpleKLResult> = Vec::new();

            if matcher.fuzzy_match("shutdown", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(
                            extension_id.clone(),
                            "@src/images/shutdown.svg".to_string(),
                        ),
                        "accent".to_string(),
                        "Shutdown".to_string(),
                        ResultAction::ExtensionAction(ExtensionAction::new(
                            extension_id.clone(),
                            "shutdown".to_string(),
                        )),
                    ),
                ))
            }

            if matcher.fuzzy_match("reboot", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(
                            extension_id.clone(),
                            "@src/images/reboot.svg".to_string(),
                        ),
                        "accent".to_string(),
                        "Reboot".to_string(),
                        ResultAction::ExtensionAction(ExtensionAction::new(
                            extension_id.clone(),
                            "reboot".to_string(),
                        )),
                    ),
                ))
            }

            if matcher.fuzzy_match("suspend", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(extension_id.clone(), "@src/images/zzz.svg".to_string()),
                        "accent".to_string(),
                        "Suspend".to_string(),
                        ResultAction::ExtensionAction(ExtensionAction::new(
                            extension_id.clone(),
                            "suspend".to_string(),
                        )),
                    ),
                ))
            }

            if matcher.fuzzy_match("hibernate", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(extension_id.clone(), "@src/images/zzz.svg".to_string()),
                        "accent".to_string(),
                        "Hibernate".to_string(),
                        ResultAction::ExtensionAction(ExtensionAction::new(
                            extension_id.clone(),
                            "hibernate".to_string(),
                        )),
                    ),
                ))
            }

            if matcher.fuzzy_match("logout", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(
                            extension_id.clone(),
                            "@src/images/logout.svg".to_string(),
                        ),
                        "accent".to_string(),
                        "Logout".to_string(),
                        ResultAction::ExtensionAction(ExtensionAction::new(
                            extension_id.clone(),
                            "logout".to_string(),
                        )),
                    ),
                ))
            }

            return_results(results);
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
                let desktop_environment_setting = Settings::get_extension_setting(
                    extension_id.clone(),
                    "desktop_environment".to_string(),
                )
                .unwrap();

                println!(
                    "extension path: {}",
                    &get_extension_path(extension_id.clone()).unwrap()
                );

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
                            .arg("qdbus org.kde.ksmserver /KSMServer logout 0 0 1")
                            .output()
                            .expect("Error running logout command");
                    }
                    "custom" => {
                        let custom_logout_setting = Settings::get_extension_setting(
                            extension_id.clone(),
                            "custom_logout".to_string(),
                        )
                        .unwrap();

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
