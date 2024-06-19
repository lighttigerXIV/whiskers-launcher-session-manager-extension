use std::process::Command;
use whiskers_launcher_rs::api::extensions::ExtensionRequest;

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
    if cfg!(target_os = "linux") {
        let mut command = String::from("");
        let preset = get_extension_setting(EXTENSION_ID, "preset").unwrap();
        let use_custom_preset = preset == "custom";

        if action == "shutdown" {
            command = if use_custom_preset {
                get_extension_setting(EXTENSION_ID, "custom-shutdown").unwrap()
            } else {
                "systemctl poweroff".to_string()
            };
        }

        if action == "restart" {
            command = if use_custom_preset {
                get_extension_setting(EXTENSION_ID, "custom-reboot").unwrap()
            } else {
                "systemctl reboot".to_string()
            };
        }

        if action == "suspend" {
            command = if use_custom_preset {
                get_extension_setting(EXTENSION_ID, "custom-suspend").unwrap()
            } else {
                "systemctl suspend".to_string()
            };
        }

        if action == "hibernate" {
            command = if use_custom_preset {
                get_extension_setting(EXTENSION_ID, "custom-hibernate").unwrap()
            } else {
                "systemctl hibernate".to_string()
            };
        }

        if action == "logout" {
            command = if use_custom_preset {
                get_extension_setting(EXTENSION_ID, "custom-logout").unwrap()
            } else {
                if preset == "kde" {
                    "qdbus org.kde.Shutdown /Shutdown  org.kde.Shutdown.logout".to_string()
                } else {
                    "gnome-session-quit --no-prompt".to_string()
                }
            };
        }

        if action == "lock" {
            command = match preset.as_str() {
                "kde" => "loginctl lock-session".to_string(),
                "gnome" => "dbus-send --type=method_call --dest=org.gnome.ScreenSaver /org/gnome/ScreenSaver org.gnome.ScreenSaver.Lock".to_string(),
                _ => get_extension_setting(EXTENSION_ID, "custom-lock").unwrap()
            }
        }

        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Error running command");
    }
}
