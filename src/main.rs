use std::process::Command;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use simple_kl_rs::{
    actions::{ExtensionAction, ResultAction},
    extensions::get_parameters,
    extensions::Function::GetResults,
    extensions::{return_results, Function::RunAction},
    results::{IconWithTextResult, SimpleKLResult},
    settings::Settings, paths::get_extension_path,
    paths::get_extension_icon
};

fn main() {
    let parameters = get_parameters();
    let function = parameters.function;
    let extension_id = "com.lighttigerxiv.session.manager".to_string();

    match function {
        GetResults => {
            let search_text = parameters.search_text.unwrap();
            let matcher = SkimMatcherV2::default();
            let mut results: Vec<SimpleKLResult> = Vec::new();

            if matcher.fuzzy_match("shutdown", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(IconWithTextResult {
                    icon: String::from(get_extension_icon(extension_id.clone())),
                    text: String::from("Shutdown"),
                    action: ResultAction::ExtensionAction(ExtensionAction::new(
                        extension_id.clone(),
                        String::from("shutdown"),
                    )),
                }))
            }

            if matcher.fuzzy_match("reboot", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(IconWithTextResult {
                    icon: String::from(get_extension_icon(extension_id.clone())),
                    text: String::from("Reboot"),
                    action: ResultAction::ExtensionAction(ExtensionAction::new(
                        extension_id.clone(),
                        String::from("reboot"),
                    )),
                }))
            }

            if matcher.fuzzy_match("suspend", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(IconWithTextResult {
                    icon: String::from(get_extension_icon(extension_id.clone())),
                    text: String::from("Suspend"),
                    action: ResultAction::ExtensionAction(ExtensionAction::new(
                        extension_id.clone(),
                        String::from("suspend"),
                    )),
                }))
            }

            if matcher.fuzzy_match("hibernate", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(IconWithTextResult {
                    icon: String::from(get_extension_icon(extension_id.clone())),
                    text: String::from("Hibernate"),
                    action: ResultAction::ExtensionAction(ExtensionAction::new(
                        extension_id.clone(),
                        String::from("hibernate"),
                    )),
                }))
            }

            if matcher.fuzzy_match("logout", &search_text).is_some() {
                results.push(SimpleKLResult::IconWithText(IconWithTextResult {
                    icon: String::from(get_extension_icon(extension_id.clone())),
                    text: String::from("Logout"),
                    action: ResultAction::ExtensionAction(ExtensionAction::new(
                        extension_id.clone(),
                        String::from("logout"),
                    )),
                }))
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

                println!("extension path: {}", &get_extension_path(extension_id.clone()).unwrap());

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
