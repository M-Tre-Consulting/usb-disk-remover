#[cfg(target_os = "windows")]
use windows::core::PCWSTR;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE};
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::{
    CreateEventW, CreateMutexW, OpenEventW, SetEvent, WaitForSingleObject, EVENT_MODIFY_STATE,
    INFINITE,
};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    AllowSetForegroundWindow, BringWindowToTop, FindWindowW, SetForegroundWindow, ShowWindow,
    SW_RESTORE,
};

#[cfg(target_os = "windows")]
use crate::utils::str_to_utf16vec;
#[cfg(target_os = "windows")]
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

#[cfg(target_os = "windows")]
const MUTEX_NAME: &str = "Local\\USBDiskRemover_SingleInstance_Mutex";
#[cfg(target_os = "windows")]
const EVENT_NAME: &str = "Local\\USBDiskRemover_SingleInstance_ShowEvent";
#[cfg(target_os = "windows")]
const WINDOW_TITLE: &str = "USB Disk Remover";

// ASFW_ANY in Win32 is ((DWORD)-1)
#[cfg(target_os = "windows")]
const ASFW_ANY: u32 = u32::MAX;

pub enum SingleInstanceStatus {
    FirstInstance(SingleInstanceGuard),
    AlreadyRunning,
}

pub struct SingleInstanceGuard {
    #[cfg(target_os = "windows")]
    mutex: HANDLE,
    #[cfg(target_os = "windows")]
    event: HANDLE,
}

impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        unsafe {
            if !self.mutex.is_invalid() {
                let _ = CloseHandle(self.mutex);
            }
            if !self.event.is_invalid() {
                let _ = CloseHandle(self.event);
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub fn acquire_single_instance(should_show_window: bool) -> SingleInstanceStatus {
    let mutex_name_w = str_to_utf16vec(MUTEX_NAME);
    let event_name_w = str_to_utf16vec(EVENT_NAME);

    unsafe {
        // Try creating the mutex with initial ownership
        let mutex = match CreateMutexW(None, true, PCWSTR(mutex_name_w.as_ptr())) {
            Ok(handle) => handle,
            Err(_) => return SingleInstanceStatus::AlreadyRunning,
        };

        // If the mutex already existed, another instance is already running
        if GetLastError() == ERROR_ALREADY_EXISTS {
            let _ = CloseHandle(mutex);

            if should_show_window {
                // Allow the running instance to bring its window to foreground
                let _ = AllowSetForegroundWindow(ASFW_ANY);

                // Signal the event to notify the primary instance to show itself
                if let Ok(event) = OpenEventW(
                    EVENT_MODIFY_STATE,
                    false,
                    PCWSTR(event_name_w.as_ptr()),
                ) {
                    let _ = SetEvent(event);
                    let _ = CloseHandle(event);
                }

                // Also attempt to find the window and restore it directly
                let title_w = str_to_utf16vec(WINDOW_TITLE);
                if let Ok(hwnd) = FindWindowW(None, PCWSTR(title_w.as_ptr())) {
                    if !hwnd.0.is_null() {
                        let _ = ShowWindow(hwnd, SW_RESTORE);
                        let _ = SetForegroundWindow(hwnd);
                    }
                }
            }

            return SingleInstanceStatus::AlreadyRunning;
        }

        // We are the first instance: create the named show event (auto-reset = false, manual_reset = false)
        let event = match CreateEventW(None, false, false, PCWSTR(event_name_w.as_ptr())) {
            Ok(handle) => handle,
            Err(_) => HANDLE::default(),
        };

        SingleInstanceStatus::FirstInstance(SingleInstanceGuard { mutex, event })
    }
}

#[cfg(not(target_os = "windows"))]
pub fn acquire_single_instance(_should_show_window: bool) -> SingleInstanceStatus {
    SingleInstanceStatus::FirstInstance(SingleInstanceGuard {})
}

#[cfg(target_os = "windows")]
pub fn bring_window_to_front(window: &slint::Window) {
    let handle = window.window_handle();
    let Ok(window_handle) = handle.window_handle() else {
        return;
    };

    if let RawWindowHandle::Win32(win32_handle) = window_handle.as_raw() {
        let hwnd = windows::Win32::Foundation::HWND(win32_handle.hwnd.get() as *mut _);
        unsafe {
            let _ = ShowWindow(hwnd, SW_RESTORE);
            let _ = BringWindowToTop(hwnd);
            let _ = SetForegroundWindow(hwnd);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn bring_window_to_front(_window: &slint::Window) {}

impl SingleInstanceGuard {
    pub fn listen_for_show_requests<W>(&self, win_weak: slint::Weak<W>)
    where
        W: slint::ComponentHandle + 'static,
    {
        #[cfg(target_os = "windows")]
        {
            if self.event.is_invalid() {
                return;
            }

            let event_raw = self.event.0 as usize;
            std::thread::spawn(move || {
                let event = HANDLE(event_raw as *mut core::ffi::c_void);
                loop {
                    let wait_res = unsafe { WaitForSingleObject(event, INFINITE) };
                    if wait_res.0 != 0 {
                        // Non-zero means failed or abandoned (WAIT_OBJECT_0 is 0)
                        break;
                    }

                    let win_weak_clone = win_weak.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(win) = win_weak_clone.upgrade() {
                            let _ = win.window().show();
                            bring_window_to_front(win.window());
                        }
                    });
                }
            });
        }
    }
}
