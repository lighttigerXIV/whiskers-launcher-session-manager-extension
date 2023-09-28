//Add this to hide commands on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
use{
    std::os::windows::process::CommandExt,
    others::FLAG_NO_WINDOW
};


use std::process::Command;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use resources::{get_logout_icon, get_reboot_icon, get_shutdown_icon, get_zzz_icon, EXTENSION_ID, get_lock_icon};
use simple_kl_rs::{
    actions::{ExtensionAction, ResultAction},
    extensions::{emit_results, get_parameters, Function},
    
    results::{IconWithTextResult, SimpleKLResult},
};
use simple_kl_rs::extensions::get_extension_setting;

pub mod resources;

fn main() {
    let parameters = get_parameters().expect("Error getting parameters");
    let function = parameters.function;

    match function {
        Function::GetResults => {
            let mut results: Vec<SimpleKLResult> = Vec::new();
            let search_text = &parameters.search_text.unwrap();
            let fuzzy_matcher = SkimMatcherV2::default();

            if fuzzy_matcher.fuzzy_match("shutdown", search_text).is_some() {
                let result = SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                    get_shutdown_icon().unwrap(),
                    "Shutdown",
                    ResultAction::ExtensionAction(ExtensionAction::new(EXTENSION_ID, "shutdown")),
                ));

                results.push(result);
            }

            if fuzzy_matcher.fuzzy_match("reboot", search_text).is_some() {
                let result = SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                    get_reboot_icon().unwrap(),
                    "Reboot",
                    ResultAction::ExtensionAction(ExtensionAction::new(EXTENSION_ID, "reboot")),
                ));

                results.push(result);
            }

            if fuzzy_matcher.fuzzy_match("suspend", search_text).is_some() {
                let result = SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                    get_zzz_icon().unwrap(),
                    "Suspend",
                    ResultAction::ExtensionAction(ExtensionAction::new(EXTENSION_ID, "suspend")),
                ));

                results.push(result);
            }

            if cfg!(target_os = "linux") {
                if fuzzy_matcher
                    .fuzzy_match("hibernate", search_text)
                    .is_some()
                {
                    let result = SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                        get_zzz_icon().unwrap(),
                        "Hibernate",
                        ResultAction::ExtensionAction(ExtensionAction::new(
                            EXTENSION_ID,
                            "hibernate",
                        )),
                    ));

                    results.push(result);
                }
            }

            if fuzzy_matcher.fuzzy_match("logout", search_text).is_some() {
                let result = SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                    get_logout_icon().unwrap(),
                    "Logout",
                    ResultAction::ExtensionAction(ExtensionAction::new(EXTENSION_ID, "logout")),
                ));

                results.push(result);
            }

            if fuzzy_matcher.fuzzy_match("lock", search_text).is_some() {
                let result = SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                    get_lock_icon().unwrap(),
                    "Lock",
                    ResultAction::ExtensionAction(ExtensionAction::new(EXTENSION_ID, "lock")),
                ));

                results.push(result);
            }

            emit_results(&results);
        }
        Function::RunAction => {
            let action = &parameters.action.unwrap();

            #[cfg(target_os = "windows")]
            if cfg!(target_os = "windows") {

                if action == "shutdown" {
                    Command::new("powershell")
                        .arg("-Command")
                        .arg("Stop-Computer")
                        .creation_flags(FLAG_NO_WINDOW)
                        .output()
                        .expect("Error running shutdown command");
                }

                if action == "reboot" {
                    Command::new("powershell")
                        .arg("-Command")
                        .arg("Restart-Computer")
                        .creation_flags(FLAG_NO_WINDOW)
                        .output()
                        .expect("Error running reboot command");
                }

                if action == "suspend" {
                    Command::new("Rundll32.exe")
                        .arg("powrprof.dll,SetSuspendState")
                        .arg("Sleep")
                        .creation_flags(FLAG_NO_WINDOW)
                        .output()
                        .expect("Error running suspend command");
                }

                if action == "logout" {
                    Command::new("shutdown.exe")
                        .arg("-l")
                        .creation_flags(FLAG_NO_WINDOW)
                        .output()
                        .expect("Error running logout command");
                }

                if action == "lock"{
                    Command::new("Rundll32.exe")
                        .arg("user32.dll,LockWorkStation")
                        .creation_flags(FLAG_NO_WINDOW)
                        .output()
                        .expect("Error running lock command");
                }
            } 
            
            #[cfg(target_os = "linux")]
            if cfg!(target_os = "linux"){
                if action == "shutdown" {
                    Command::new("sh")
                        .arg("-c")
                        .arg("systemctl poweroff")
                        .output()
                        .expect("Error running poweroff command");
                }

                if action == "reboot" {
                    Command::new("sh")
                        .arg("-c")
                        .arg("systemctl reboot")
                        .output()
                        .expect("Error running reboot command");
                }

                if action == "suspend" {
                    Command::new("sh")
                        .arg("-c")
                        .arg("systemctl suspend")
                        .output()
                        .expect("Error running suspend command");
                }

                if action == "hibernate"{
                    Command::new("sh")
                        .arg("-c")
                        .arg("systemctl hibernate")
                        .output()
                        .expect("Error running shutdown command");
                }

                if action == "logout" {
                    let de_setting =
                        get_extension_setting(EXTENSION_ID, "desktop_environment")
                            .expect("Error getting desktop environment setting");

                    if de_setting == "gnome"{
                        Command::new("sh")
                            .arg("-c")
                            .arg("gnome-session-quit --no-prompt")
                            .output()
                            .expect("Error running logout command");
                    }

                    if de_setting == "kde"{
                        Command::new("sh")
                            .arg("-c")
                            .arg("qdbus org.kde.ksmserver /KSMServer logout 0 0 2")
                            .output()
                            .expect("Error running logout command");
                    }

                    if de_setting == "custom"{
                        let logout_setting =
                            get_extension_setting(EXTENSION_ID, "custom_logout")
                                .expect("Error getting custom logout setting");

                        Command::new("sh")
                            .arg("-c")
                            .arg(logout_setting)
                            .output()
                            .expect("Error running logout command");
                    }
                }

                if action == "lock"{
                    let de_setting =
                        get_extension_setting(EXTENSION_ID, "desktop_environment")
                            .expect("Error getting desktop environment setting");

                    if de_setting == "gnome"{
                        Command::new("sh")
                            .arg("-c")
                            .arg("dbus-send --type=method_call --dest=org.gnome.ScreenSaver /org/gnome/ScreenSaver org.gnome.ScreenSaver.Lock")
                            .output()
                            .expect("Error running logout command");
                    }

                    if de_setting == "kde"{
                        Command::new("sh")
                            .arg("-c")
                            .arg("loginctl lock-session")
                            .output()
                            .expect("Error running logout command");
                    }

                    if de_setting == "custom"{
                        let lock_setting =
                           get_extension_setting(EXTENSION_ID, "custom_lock")
                               .expect("Error getting custom lock setting");

                        Command::new("sh")
                            .arg("-c")
                            .arg(lock_setting)
                            .output()
                            .expect("Error running logout command");
                    }
                }
            }
        }
    }
}
