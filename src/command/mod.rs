use whiskers_launcher_core::features::extensions::get_extension_setting;

use crate::ID;

pub struct SessionCommands {
    pub shutdown: String,
    pub restart: String,
    pub suspend: String,
    pub hibernate: String,
    pub logout: String,
    pub lock: String,
}

impl SessionCommands {
    pub fn new(
        shutdown: &str,
        restart: &str,
        suspend: &str,
        hibernate: &str,
        logout: &str,
        lock: &str,
    ) -> Self {
        Self {
            shutdown: shutdown.to_string(),
            restart: restart.to_string(),
            suspend: suspend.to_string(),
            hibernate: hibernate.to_string(),
            logout: logout.to_string(),
            lock: lock.to_string(),
        }
    }
}

pub fn get_kde_commands() -> SessionCommands {
    SessionCommands::new(
        "systemctl poweroff",
        "systemctl reboot",
        "systemctl suspend",
        "systemctl hibernate",
        "qdbus6 org.kde.Shutdown /Shutdown  org.kde.Shutdown.logout",
        "loginctl lock-session",
    )
}

pub fn get_gnome_commands() -> SessionCommands {
    SessionCommands::new(
        "systemctl poweroff",
        "systemctl reboot",
        "systemctl suspend",
        "systemctl hibernate",
        "gnome-session-quit --no-prompt",
        "dbus-send --type=method_call --dest=org.gnome.ScreenSaver /org/gnome/ScreenSaver org.gnome.ScreenSaver.Lock",
    )
}

pub fn get_cinnamon_commands() -> SessionCommands {
    SessionCommands::new(
        "systemctl poweroff",
        "systemctl reboot",
        "systemctl suspend",
        "systemctl hibernate",
        "cinnamon-session-quit --logout --force",
        "cinnamon-screensaver-command --lock",
    )
}

pub fn get_hyprland_commands() -> SessionCommands {
    SessionCommands::new(
        "systemctl poweroff",
        "systemctl reboot",
        "systemctl suspend",
        "systemctl hibernate",
        "hyprctl dispatch exit",
        "hyprlock",
    )
}

pub fn get_custom_commands() -> SessionCommands {
    SessionCommands::new(
        &get_extension_setting(ID, "custom-shutdown").unwrap(),
        &get_extension_setting(ID, "custom-reboot").unwrap(),
        &get_extension_setting(ID, "custom-suspend").unwrap(),
        &get_extension_setting(ID, "custom-hibernate").unwrap(),
        &get_extension_setting(ID, "custom-logout").unwrap(),
        &get_extension_setting(ID, "custom-lock").unwrap(),
    )
}

pub fn get_default_commands() -> SessionCommands {
    SessionCommands::new(
        "systemctl poweroff",
        "systemctl reboot",
        "systemctl suspend",
        "systemctl hibernate",
        "notify-send 'Environment not coded. Please open a issue'",
        "notify-send 'Environment not coded. Please open a issue'",
    )
}

pub fn get_session_commands(environment: &str) -> SessionCommands {
    match environment.to_lowercase().as_str() {
        "kde" => get_kde_commands(),
        "gnome" => get_gnome_commands(),
        "x-cinnamon" => get_cinnamon_commands(),
        "hyprland" => get_hyprland_commands(),
        _ => get_default_commands(),
    }
}
