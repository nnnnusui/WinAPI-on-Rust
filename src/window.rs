use std::ptr;
use winapi::um::winuser::{CreateWindowExW, CW_USEDEFAULT, WS_OVERLAPPEDWINDOW};
use winapi::{
    ctypes::c_int,
    shared::{minwindef::BOOL, windef::HWND},
    um::winuser::{ShowWindow, UpdateWindow},
};

pub struct Window {
    pub hwnd: HWND,
}

impl Window {
    pub unsafe fn create(class_name: &[u16], window_name: &[u16]) -> Result<Window, String> {
        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if hwnd.is_null() {
            return Err("CreateWindowExW return null".to_owned());
        }
        Ok(Window { hwnd })
    }
    pub unsafe fn show(&self, n_cmd_show: c_int) -> BOOL {
        ShowWindow(self.hwnd, n_cmd_show)
    }
    pub unsafe fn update(&self) -> BOOL {
        UpdateWindow(self.hwnd)
    }
}
