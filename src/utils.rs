/// Returns true if the bit in position `bit` for `value` is set.
pub fn is_bit_set(value: u32, bit: u8) -> bool {
    (value & (1 << bit)) != 0
}

/// Converts a string literal to an UTF16 word array
pub fn str_to_utf16vec(value: &str) -> Vec<u16> {
    value.encode_utf16().chain([0u16]).collect::<Vec<u16>>()
}

/// Detects if the current OS is Windows 11 (NT build >= 22000) or Windows 10
#[cfg(target_os = "windows")]
pub fn is_windows_11_or_greater() -> bool {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_LOCAL_MACHINE, KEY_READ,
    };

    let subkey = str_to_utf16vec(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion");
    let mut hkey = HKEY::default();
    let status = unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey.as_ptr()),
            None,
            KEY_READ,
            &mut hkey,
        )
    };

    if status.is_err() {
        return true;
    }

    let val_name = str_to_utf16vec("CurrentBuildNumber");
    let mut buffer = [0u16; 32];
    let mut buf_size = (buffer.len() * 2) as u32;

    let res = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(val_name.as_ptr()),
            None,
            None,
            Some(buffer.as_mut_ptr() as *mut u8),
            Some(&mut buf_size),
        )
    };

    unsafe {
        let _ = RegCloseKey(hkey);
    }

    if res.is_ok() {
        let null_pos = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        let build_str = String::from_utf16_lossy(&buffer[..null_pos]);
        if let Ok(build_num) = build_str.trim().parse::<u32>() {
            return build_num >= 22000;
        }
    }

    true
}

#[cfg(not(target_os = "windows"))]
pub fn is_windows_11_or_greater() -> bool {
    true
}
