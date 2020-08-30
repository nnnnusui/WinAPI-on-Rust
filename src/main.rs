use std::io::Error;
use std::mem;
use std::ptr;

use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM, LRESULT, TRUE, UINT, WPARAM},
        windef::{HBRUSH, HDC, HMONITOR, HWND, LPRECT, RECT},
    },
    um::{
        wingdi::{GetStockObject, WHITE_BRUSH},
        winuser::{CreateWindowExW, CS_HREDRAW, CS_VREDRAW, DefWindowProcW,
                  DispatchMessageW, GetMessageW, IDC_ARROW, IDI_APPLICATION,
                  LoadCursorW, LoadIconW, MSG, PostQuitMessage,
                  RegisterClassW, ShowWindow, SW_NORMAL, TranslateMessage,
                  UpdateWindow, WM_DESTROY, WNDCLASSW, WS_OVERLAPPEDWINDOW},
    },
};
use winapi::um::winuser::{CW_USEDEFAULT, EnumDisplayMonitors, GetMonitorInfoW, MonitorFromPoint, MONITORINFOEXW, SW_SHOWNORMAL};

fn main() {
    unsafe {
        for monitor in enumerate_monitors() {
            // Convert the WCHAR[] to a unicode OsString
            use std::ffi::OsString;
            use std::os::windows::ffi::OsStringExt;
            let name = match &monitor.szDevice[..].iter().position(|c| *c == 0) {
                Some(len) => OsString::from_wide(&monitor.szDevice[0..*len]),
                None => OsString::from_wide(&monitor.szDevice[0..monitor.szDevice.len()]),
            };

            // Print some information to the console
            println!("Display name = {}", name.to_str().unwrap());
            println!("    Left: {}", monitor.rcWork.left);
            println!("   Right: {}", monitor.rcWork.right);
            println!("     Top: {}", monitor.rcWork.top);
            println!("  Bottom: {}", monitor.rcWork.bottom);
        }
    }
}

fn enumerate_monitors() -> Vec<MONITORINFOEXW> {
    // Define the vector where we will store the result
    let mut monitors = Vec::<MONITORINFOEXW>::new();
    let userdata = &mut monitors as *mut _;

    let result = unsafe {
        EnumDisplayMonitors(
            ptr::null_mut(),
            ptr::null(),
            Some(enumerate_monitors_callback),
            userdata as LPARAM,
        )
    };

    if result != TRUE {
        // Get the last error for the current thread.
        // This is analogous to calling the Win32 API GetLastError.
        panic!("Could not enumerate monitors: {}", Error::last_os_error());
    }

    monitors
}
unsafe extern "system" fn enumerate_monitors_callback(
    monitor: HMONITOR,
    _: HDC,
    _: LPRECT,
    userdata: LPARAM,
) -> BOOL {
    // Get the userdata where we will store the result
    let monitors: &mut Vec<MONITORINFOEXW> = mem::transmute(userdata);

    // Initialize the MONITORINFOEXW structure and get a pointer to it
    let mut monitor_info: MONITORINFOEXW = mem::zeroed();
    monitor_info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
    let monitor_info_ptr = <*mut _>::cast(&mut monitor_info);

    // Call the GetMonitorInfoW win32 API
    let result = GetMonitorInfoW(monitor, monitor_info_ptr);
    if result == TRUE {
        // Push the information we received to userdata
        monitors.push(monitor_info);
    }

    TRUE
}
