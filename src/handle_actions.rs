use std::{
    env,
    process::{exit, Command},
};
use whiskers_launcher_rs::api::extensions::ExtensionRequest;

use crate::command::{get_custom_commands, get_session_commands};

#[cfg(target_os = "linux")]
use {crate::EXTENSION_ID, whiskers_launcher_rs::api::extensions::get_extension_setting};

#[cfg(target_os = "windows")]
use {std::os::windows::process::CommandExt, whiskers_launcher_rs::utils::FLAG_NO_WINDOW};

pub fn handle_actions(request: ExtensionRequest) {
    let action = request.extension_action.unwrap();

    #[cfg(target_os = "windows")]
    {
        if action == "shutdown" {
            Command::new("powershell")
                .arg("-Command")
                .arg("Stop-Computer")
                .creation_flags(FLAG_NO_WINDOW)
                .output()
                .expect("Error running shutdown command");
        }

        if action == "restart" {
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

        if action == "lock" {
            Command::new("Rundll32.exe")
                .arg("user32.dll,LockWorkStation")
                .creation_flags(FLAG_NO_WINDOW)
                .output()
                .expect("Error running lock command");
        }
    }

    #[cfg(target_os = "linux")]
    {
        let preset = get_extension_setting(EXTENSION_ID, "preset").unwrap();

        let session_commands = match preset == "auto" {
            true => {
                let env =
                    env::var("XDG_CURRENT_DESKTOP").expect("Error getting session environment");

                get_session_commands(&env)
            }
            false => {
                if preset == "custom" {
                    get_custom_commands()
                } else {
                    get_session_commands(&preset)
                }
            }
        };

        let command = match action.as_str() {
            "shutdown" => &session_commands.shutdown,
            "restart" => &session_commands.restart,
            "suspend" => &session_commands.suspend,
            "hibernate" => &session_commands.hibernate,
            "logout" => &session_commands.logout,
            "lock" => &session_commands.lock,
            _ => {
                panic!("HUH? 😺");
            }
        };

        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Error running command");

        exit(0);
    }
}
