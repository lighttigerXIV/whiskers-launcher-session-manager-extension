use std::path::PathBuf;

use simple_kl_rs::paths::get_extension_directory;

pub const EXTENSION_ID: &str = "com-lighttigerxiv-session-manager";

pub fn get_icons_folder() -> Option<PathBuf>{

    let mut path = get_extension_directory(EXTENSION_ID)?;
    path.push("src");
    path.push("resources");
    path.push("icons");

    return Some(path)
}

pub fn get_shutdown_icon()->Option<PathBuf>{

    let mut path = get_icons_folder()?;
    path.push("shutdown.svg");

    return Some(path);
}

pub fn get_reboot_icon()->Option<PathBuf>{

    let mut path = get_icons_folder()?;
    path.push("reboot.svg");

    return Some(path);
}

pub fn get_zzz_icon()->Option<PathBuf>{

    let mut path = get_icons_folder()?;
    path.push("zzz.svg");

    return Some(path);
}

pub fn get_logout_icon()->Option<PathBuf>{

    let mut path = get_icons_folder()?;
    path.push("logout.svg");

    return Some(path);
}

pub fn get_lock_icon()->Option<PathBuf>{

    let mut path = get_icons_folder()?;
    path.push("lock.svg");

    return Some(path);
}

