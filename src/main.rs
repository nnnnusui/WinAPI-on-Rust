use std::mem;
use std::ptr;
use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM, TRUE},
        windef::HWND,
    },
    um::winuser::{EnumWindows, GetWindowTextW},
};

fn main() {
    for window in enumerate_windows() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        let mut buf = [0u16; 1024];
        let success = unsafe { GetWindowTextW(window, &mut buf[0], 1024) > 0 };
        if success {
            if let Some(name) = OsString::from_wide(&buf[..]).to_str() {
                println!("{}", name)
            }
        }
    }
}

fn enumerate_windows() -> Vec<HWND> {
    let mut windows = Vec::<HWND>::new();
    let userdata = &mut windows as *mut _;
    let result = unsafe { EnumWindows(Some(enumerate_windows_callback), userdata as LPARAM) };

    windows
}

unsafe extern "system" fn enumerate_windows_callback(hwnd: HWND, userdata: LPARAM) -> BOOL {
    // Get the userdata where we will store the result
    let windows: &mut Vec<HWND> = mem::transmute(userdata);
    windows.push(hwnd);

    TRUE
}
