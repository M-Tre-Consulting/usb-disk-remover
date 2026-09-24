use crate::utils::str_to_utf16vec;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use windows::core::PCWSTR;
use windows::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, HKEY,
    HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_SZ,
};

const REG_RUN_SUBKEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const REG_APP_NAME: &str = "USBDiskRemover";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub start_with_windows: bool,
    pub start_minimized: bool,
    pub close_to_tray: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            start_with_windows: false,
            start_minimized: false,
            close_to_tray: true,
        }
    }
}

fn get_settings_path() -> Option<PathBuf> {
    let appdata = std::env::var_os("APPDATA")?;
    let mut path = PathBuf::from(appdata);
    path.push("USBDiskRemover");
    std::fs::create_dir_all(&path).ok()?;
    path.push("settings.json");
    Some(path)
}

pub fn load_settings() -> AppSettings {
    let mut settings = if let Some(path) = get_settings_path() {
        if let Ok(content) = std::fs::read_to_string(&path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            AppSettings::default()
        }
    } else {
        AppSettings::default()
    };

    // Keep Windows startup state in sync with actual Registry
    settings.start_with_windows = is_autostart_enabled();
    settings
}

pub fn save_settings(settings: &AppSettings) {
    if let Some(path) = get_settings_path() {
        if let Ok(json) = serde_json::to_string_pretty(settings) {
            let _ = std::fs::write(path, json);
        }
    }

    let _ = set_autostart(settings.start_with_windows, settings.start_minimized);
}

pub fn is_autostart_enabled() -> bool {
    let subkey = str_to_utf16vec(REG_RUN_SUBKEY);
    let mut hkey = HKEY::default();

    let status = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            None,
            KEY_READ,
            &mut hkey,
        )
    };

    if status.is_err() {
        return false;
    }

    let val_name = str_to_utf16vec(REG_APP_NAME);
    let result = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(val_name.as_ptr()),
            None,
            None,
            None,
            None,
        )
    };

    unsafe {
        let _ = RegCloseKey(hkey);
    }

    result.is_ok()
}

pub fn set_autostart(enable: bool, start_minimized: bool) -> Result<(), String> {
    let subkey = str_to_utf16vec(REG_RUN_SUBKEY);
    let mut hkey = HKEY::default();

    let status = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            None,
            KEY_READ | KEY_WRITE,
            &mut hkey,
        )
    };

    if status.is_err() {
        return Err("Impossibile aprire il registro di configurazione di Windows".into());
    }

    let val_name = str_to_utf16vec(REG_APP_NAME);

    let res = if enable {
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Impossibile determinare il percorso dell'eseguibile: {e}"))?;
        let exe_str = exe_path.to_string_lossy();

        let cmd = if start_minimized {
            format!("\"{}\" --minimized", exe_str)
        } else {
            format!("\"{}\"", exe_str)
        };

        let cmd_utf16 = str_to_utf16vec(&cmd);

        unsafe {
            RegSetValueExW(
                hkey,
                PCWSTR(val_name.as_ptr()),
                None,
                REG_SZ,
                Some(std::slice::from_raw_parts(
                    cmd_utf16.as_ptr() as *const u8,
                    cmd_utf16.len() * 2,
                )),
            )
        }
    } else {
        unsafe { RegDeleteValueW(hkey, PCWSTR(val_name.as_ptr())) }
    };

    unsafe {
        let _ = RegCloseKey(hkey);
    }

    if res.0 != 0 {
        return Err(format!("Errore aggiornamento registro: codice {}", res.0));
    }
    Ok(())
}
