use std::process::Command;

use whiskers_launcher_rs::api::extensions::{get_extension_setting, Context};

use crate::EXTENSION_ID;

#[cfg(target_os = "windows")]
use{
    std::os::windows::process::CommandExt,
    others::FLAG_NO_WINDOW
};

pub fn handle_actions(context: Context) {
    let action = context.extension_action.unwrap();


    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        if action == "shut_down" {
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

        if action == "lock" {
            Command::new("Rundll32.exe")
                .arg("user32.dll,LockWorkStation")
                .creation_flags(FLAG_NO_WINDOW)
                .output()
                .expect("Error running lock command");
        }
    }

    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {

        if action == "shut_down" {
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

        if action == "hibernate" {
            Command::new("sh")
                .arg("-c")
                .arg("systemctl hibernate")
                .output()
                .expect("Error running shutdown command");
        }

        if action == "logout" {
            let desktop_environment = get_extension_setting(EXTENSION_ID, "desktop_environment")
                .expect("Error getting desktop environment setting");

            if desktop_environment == "gnome" {
                Command::new("sh")
                    .arg("-c")
                    .arg("gnome-session-quit --no-prompt")
                    .output()
                    .expect("Error running logout command");
            }

            if desktop_environment == "kde" {
                Command::new("sh")
                    .arg("-c")
                    .arg("qdbus org.kde.ksmserver /KSMServer logout 0 0 2")
                    .output()
                    .expect("Error running logout command");
            }

            if desktop_environment == "other" {
                let logout_setting = get_extension_setting(EXTENSION_ID, "custom_logout")
                    .expect("Error getting custom logout setting");

                Command::new("sh")
                    .arg("-c")
                    .arg(logout_setting)
                    .output()
                    .expect("Error running logout command");
            }
        }

        if action == "lock" {
            let desktop_environment = get_extension_setting(EXTENSION_ID, "desktop_environment")
                .expect("Error getting desktop environment setting");

            if desktop_environment == "gnome" {
                Command::new("sh")
                            .arg("-c")
                            .arg("dbus-send --type=method_call --dest=org.gnome.ScreenSaver /org/gnome/ScreenSaver org.gnome.ScreenSaver.Lock")
                            .output()
                            .expect("Error running logout command");
            }

            if desktop_environment == "kde" {
                Command::new("sh")
                    .arg("-c")
                    .arg("loginctl lock-session")
                    .output()
                    .expect("Error running logout command");
            }

            if desktop_environment == "other" {
                let lock_setting = get_extension_setting(EXTENSION_ID, "custom_lock")
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
