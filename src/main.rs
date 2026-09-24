#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod drives;
mod eject;
mod i18n;
mod settings;
mod utils;

use drives::{enumerate_drives, BusType, RemovableDrive};
use eject::eject_drive;
use i18n::Language;
use settings::{load_settings, save_settings, AppSettings};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

slint::include_modules!();

#[cfg(target_os = "windows")]
fn apply_windows_styling(window: &slint::Window) {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows::Win32::Graphics::Dwm::{
        DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE,
    };

    let handle = window.window_handle();
    let Ok(window_handle) = handle.window_handle() else {
        return;
    };

    if let RawWindowHandle::Win32(win32_handle) = window_handle.as_raw() {
        let hwnd = windows::Win32::Foundation::HWND(win32_handle.hwnd.get() as *mut _);
        unsafe {
            let dark: u32 = 1;
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &dark as *const u32 as *const _,
                std::mem::size_of::<u32>() as u32,
            );
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn apply_windows_styling(_window: &slint::Window) {}

fn drive_to_item(drive: &RemovableDrive) -> DriveItem {
    let bus_str = match drive.bus_type {
        BusType::Usb => "USB",
        BusType::Firewire => "1394",
        BusType::Unknown => "Other",
    };

    DriveItem {
        mount_point: drive.mount_point.clone().into(),
        label: drive.label.clone().into(),
        vendor: drive.vendor.clone().into(),
        product: drive.product.clone().into(),
        bus_type: bus_str.into(),
    }
}

fn load_drives_async(
    win_weak: slint::Weak<MainWindow>,
    is_busy: Arc<AtomicBool>,
    current_lang: Arc<Mutex<Language>>,
) {
    if is_busy.swap(true, Ordering::SeqCst) {
        return;
    }

    let lang = current_lang.lock().map(|l| *l).unwrap_or_default();
    if let Some(win) = win_weak.upgrade() {
        win.set_is_loading(true);
        win.set_status_text(lang.get_strings().scanning);
        win.set_status_kind("idle".into());
    }

    let is_busy_clone = Arc::clone(&is_busy);
    let lang_clone = Arc::clone(&current_lang);
    std::thread::spawn(move || {
        let drives = enumerate_drives();
        let items: Vec<DriveItem> = drives.iter().map(drive_to_item).collect();
        let count = items.len();

        let _ = slint::invoke_from_event_loop(move || {
            let active_lang = lang_clone.lock().map(|l| *l).unwrap_or_default();
            if let Some(win) = win_weak.upgrade() {
                let model = Rc::new(VecModel::from(items));
                win.set_drives(ModelRc::from(model));
                win.set_selected_index(-1);
                win.set_is_loading(false);

                if count == 0 {
                    win.set_status_text(active_lang.get_strings().no_drives_title);
                    win.set_status_kind("idle".into());
                } else {
                    let msg = active_lang.drives_detected(count);
                    win.set_status_text(msg.into());
                    win.set_status_kind("idle".into());
                }
            }
            is_busy_clone.store(false, Ordering::SeqCst);
        });
    });
}

fn eject_drive_async(
    mount_point: String,
    win_weak: slint::Weak<MainWindow>,
    is_busy: Arc<AtomicBool>,
    current_lang: Arc<Mutex<Language>>,
) {
    if is_busy.swap(true, Ordering::SeqCst) {
        return;
    }

    let lang = current_lang.lock().map(|l| *l).unwrap_or_default();
    if let Some(win) = win_weak.upgrade() {
        win.set_is_ejecting(true);
        let msg = lang.safely_removing(&mount_point);
        win.set_status_text(msg.into());
        win.set_status_kind("idle".into());
    }

    let is_busy_clone = Arc::clone(&is_busy);
    let lang_clone = Arc::clone(&current_lang);
    std::thread::spawn(move || {
        let all_drives = enumerate_drives();
        let target = all_drives.iter().find(|d| {
            d.mount_point.trim_end_matches('\\') == mount_point.trim_end_matches('\\')
        });

        let active_lang = lang_clone.lock().map(|l| *l).unwrap_or_default();
        let (success, message) = match target {
            Some(drive) => match eject_drive(drive) {
                Ok(_) => (true, active_lang.safely_removed(&mount_point)),
                Err(err) => (false, active_lang.eject_error(&mount_point, &format!("{err:?}"))),
            },
            None => (false, active_lang.drive_not_found(&mount_point)),
        };

        let refreshed = enumerate_drives();
        let items: Vec<DriveItem> = refreshed.iter().map(drive_to_item).collect();

        let _ = slint::invoke_from_event_loop(move || {
            if let Some(win) = win_weak.upgrade() {
                let model = Rc::new(VecModel::from(items));
                win.set_drives(ModelRc::from(model));
                win.set_selected_index(-1);
                win.set_is_ejecting(false);
                win.set_status_text(message.into());
                win.set_status_kind(if success { "ok".into() } else { "error".into() });
            }
            is_busy_clone.store(false, Ordering::SeqCst);
        });
    });
}

fn update_tray_strings(tray: &AppTray, strings: &I18nStrings) {
    tray.set_title_open(strings.tray_open.clone());
    tray.set_title_settings(strings.tray_settings.clone());
    tray.set_title_about(strings.tray_about.clone());
    tray.set_title_quit(strings.tray_quit.clone());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_window = MainWindow::new()?;
    let tray = AppTray::new()?;

    apply_windows_styling(main_window.window());

    // Set application version from Cargo
    main_window.set_app_version(env!("CARGO_PKG_VERSION").into());

    // Load persisted settings
    let initial_settings = load_settings();
    let initial_lang = Language::from_code(&initial_settings.language);
    let initial_strings = initial_lang.get_strings();

    main_window.set_t(initial_strings.clone());
    main_window.set_selected_language_index(initial_lang.to_index() as i32);
    main_window.set_status_text(initial_strings.ready.clone());

    update_tray_strings(&tray, &initial_strings);

    main_window.set_setting_start_with_windows(initial_settings.start_with_windows);
    main_window.set_setting_start_minimized(initial_settings.start_minimized);
    main_window.set_setting_close_to_tray(initial_settings.close_to_tray);

    let settings_state = Arc::new(Mutex::new(initial_settings));
    let current_lang = Arc::new(Mutex::new(initial_lang));

    // Handle close button (X) according to close_to_tray setting
    let settings_close = Arc::clone(&settings_state);
    main_window.window().on_close_requested(move || {
        let close_to_tray = settings_close
            .lock()
            .map(|s| s.close_to_tray)
            .unwrap_or(true);

        if close_to_tray {
            slint::CloseRequestResponse::HideWindow
        } else {
            let _ = slint::quit_event_loop();
            slint::CloseRequestResponse::HideWindow
        }
    });

    let is_busy = Arc::new(AtomicBool::new(false));

    // ── Tray Callbacks ────────────────────────────────────────────────────────
    let win_weak_tray = main_window.as_weak();
    tray.on_toggle_window(move || {
        if let Some(win) = win_weak_tray.upgrade() {
            if win.window().is_visible() {
                let _ = win.window().hide();
            } else {
                let _ = win.window().show();
            }
        }
    });

    let win_weak_tray_settings = main_window.as_weak();
    tray.on_show_settings(move || {
        if let Some(win) = win_weak_tray_settings.upgrade() {
            win.set_show_settings(true);
            win.set_show_about(false);
            let _ = win.window().show();
        }
    });

    let win_weak_about = main_window.as_weak();
    tray.on_show_about(move || {
        if let Some(win) = win_weak_about.upgrade() {
            win.set_show_about(true);
            win.set_show_settings(false);
            let _ = win.window().show();
        }
    });

    tray.on_quit(move || {
        let _ = slint::quit_event_loop();
    });

    // ── Language Selector Callback ───────────────────────────────────────────
    let win_weak_lang = main_window.as_weak();
    let tray_weak = tray.as_weak();
    let current_lang_select = Arc::clone(&current_lang);
    let settings_select = Arc::clone(&settings_state);
    main_window.on_select_language(move |idx| {
        let new_lang = Language::from_index(idx as usize);
        if let Ok(mut l) = current_lang_select.lock() {
            *l = new_lang;
        }
        if let Ok(mut s) = settings_select.lock() {
            s.language = new_lang.to_code().to_string();
            settings::save_settings(&s);
        }
        let strings = new_lang.get_strings();
        if let Some(win) = win_weak_lang.upgrade() {
            win.set_t(strings.clone());
            win.set_selected_language_index(idx);
        }
        if let Some(tray) = tray_weak.upgrade() {
            update_tray_strings(&tray, &strings);
        }
    });

    // ── Window Callbacks ──────────────────────────────────────────────────────
    let win_weak = main_window.as_weak();
    let is_busy_refresh = Arc::clone(&is_busy);
    let current_lang_refresh = Arc::clone(&current_lang);
    main_window.on_refresh_drives(move || {
        load_drives_async(
            win_weak.clone(),
            Arc::clone(&is_busy_refresh),
            Arc::clone(&current_lang_refresh),
        );
    });

    let win_weak = main_window.as_weak();
    main_window.on_select_drive(move |idx| {
        if let Some(win) = win_weak.upgrade() {
            win.set_selected_index(idx);
        }
    });

    let win_weak = main_window.as_weak();
    let is_busy_eject = Arc::clone(&is_busy);
    let current_lang_eject = Arc::clone(&current_lang);
    main_window.on_eject_drive(move |mount_point| {
        eject_drive_async(
            mount_point.to_string(),
            win_weak.clone(),
            Arc::clone(&is_busy_eject),
            Arc::clone(&current_lang_eject),
        );
    });

    let win_weak = main_window.as_weak();
    let is_busy_sel = Arc::clone(&is_busy);
    let current_lang_sel = Arc::clone(&current_lang);
    main_window.on_eject_selected(move || {
        if let Some(win) = win_weak.upgrade() {
            let idx = win.get_selected_index();
            if idx >= 0 {
                let drives_model = win.get_drives();
                if let Some(item) = drives_model.row_data(idx as usize) {
                    eject_drive_async(
                        item.mount_point.to_string(),
                        win_weak.clone(),
                        Arc::clone(&is_busy_sel),
                        Arc::clone(&current_lang_sel),
                    );
                }
            }
        }
    });

    let win_weak = main_window.as_weak();
    main_window.on_toggle_about(move || {
        if let Some(win) = win_weak.upgrade() {
            let current = win.get_show_about();
            win.set_show_about(!current);
            if !current {
                win.set_show_settings(false);
            }
        }
    });

    let win_weak = main_window.as_weak();
    main_window.on_toggle_settings(move || {
        if let Some(win) = win_weak.upgrade() {
            let current = win.get_show_settings();
            win.set_show_settings(!current);
            if !current {
                win.set_show_about(false);
            }
        }
    });

    let settings_save = Arc::clone(&settings_state);
    let current_lang_save = Arc::clone(&current_lang);
    main_window.on_save_settings(move |start_win, start_min, close_tray| {
        let lang_code = current_lang_save
            .lock()
            .map(|l| l.to_code().to_string())
            .unwrap_or_else(|_| "en".to_string());

        let new_settings = AppSettings {
            start_with_windows: start_win,
            start_minimized: start_min,
            close_to_tray: close_tray,
            language: lang_code,
        };

        if let Ok(mut s) = settings_save.lock() {
            *s = new_settings.clone();
        }

        save_settings(&new_settings);
    });

    // Initial drive scan
    load_drives_async(
        main_window.as_weak(),
        Arc::clone(&is_busy),
        Arc::clone(&current_lang),
    );

    // Check if launched with --minimized flag
    let start_minimized = std::env::args().any(|arg| arg == "--minimized");
    if !start_minimized {
        main_window.show()?;
    }

    slint::run_event_loop()?;

    Ok(())
}
