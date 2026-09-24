#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod drives;
mod eject;
mod utils;

use drives::{enumerate_drives, BusType, RemovableDrive};
use eject::eject_drive;
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
        BusType::Unknown => "Altro",
    };

    DriveItem {
        mount_point: drive.mount_point.clone().into(),
        label: drive.label.clone().into(),
        vendor: drive.vendor.clone().into(),
        product: drive.product.clone().into(),
        bus_type: bus_str.into(),
    }
}

fn load_drives_async(win_weak: slint::Weak<MainWindow>, is_busy: Arc<AtomicBool>) {
    if is_busy.swap(true, Ordering::SeqCst) {
        return;
    }

    if let Some(win) = win_weak.upgrade() {
        win.set_is_loading(true);
        win.set_status_text("Scansione unità in corso...".into());
        win.set_status_kind("idle".into());
    }

    let is_busy_clone = Arc::clone(&is_busy);
    std::thread::spawn(move || {
        let drives = enumerate_drives();
        let items: Vec<DriveItem> = drives.iter().map(drive_to_item).collect();
        let count = items.len();

        let _ = slint::invoke_from_event_loop(move || {
            if let Some(win) = win_weak.upgrade() {
                let model = Rc::new(VecModel::from(items));
                win.set_drives(ModelRc::from(model));
                win.set_selected_index(-1);
                win.set_is_loading(false);

                if count == 0 {
                    win.set_status_text("Nessuna unità rimovibile rilevata.".into());
                    win.set_status_kind("idle".into());
                } else {
                    let msg = if count == 1 {
                        "1 unità rilevata.".to_string()
                    } else {
                        format!("{} unità rilevate.", count)
                    };
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
) {
    if is_busy.swap(true, Ordering::SeqCst) {
        return;
    }

    if let Some(win) = win_weak.upgrade() {
        win.set_is_ejecting(true);
        let msg = format!("Rimozione sicura di {} in corso...", mount_point);
        win.set_status_text(msg.into());
        win.set_status_kind("idle".into());
    }

    let is_busy_clone = Arc::clone(&is_busy);
    std::thread::spawn(move || {
        let all_drives = enumerate_drives();
        let target = all_drives.iter().find(|d| {
            d.mount_point.trim_end_matches('\\') == mount_point.trim_end_matches('\\')
        });

        let (success, message) = match target {
            Some(drive) => match eject_drive(drive) {
                Ok(_) => (true, format!("{} rimossa con successo.", mount_point)),
                Err(err) => (false, format!("Errore espulsione {}: {:?}", mount_point, err)),
            },
            None => (false, format!("Unità {} non trovata.", mount_point)),
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_window = MainWindow::new()?;
    let tray = AppTray::new()?;

    apply_windows_styling(main_window.window());

    // Intercept window close button (X) to hide to tray instead of terminating
    main_window
        .window()
        .on_close_requested(|| slint::CloseRequestResponse::HideWindow);

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

    let win_weak_about = main_window.as_weak();
    tray.on_show_about(move || {
        if let Some(win) = win_weak_about.upgrade() {
            win.set_show_about(true);
            let _ = win.window().show();
        }
    });

    tray.on_quit(move || {
        let _ = slint::quit_event_loop();
    });

    // ── Window Callbacks ──────────────────────────────────────────────────────
    let win_weak = main_window.as_weak();
    let is_busy_refresh = Arc::clone(&is_busy);
    main_window.on_refresh_drives(move || {
        load_drives_async(win_weak.clone(), Arc::clone(&is_busy_refresh));
    });

    let win_weak = main_window.as_weak();
    main_window.on_select_drive(move |idx| {
        if let Some(win) = win_weak.upgrade() {
            win.set_selected_index(idx);
        }
    });

    let win_weak = main_window.as_weak();
    let is_busy_eject = Arc::clone(&is_busy);
    main_window.on_eject_drive(move |mount_point| {
        eject_drive_async(
            mount_point.to_string(),
            win_weak.clone(),
            Arc::clone(&is_busy_eject),
        );
    });

    let win_weak = main_window.as_weak();
    let is_busy_sel = Arc::clone(&is_busy);
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
        }
    });

    // Initial drive scan
    load_drives_async(main_window.as_weak(), Arc::clone(&is_busy));

    main_window.show()?;
    slint::run_event_loop()?;

    Ok(())
}
