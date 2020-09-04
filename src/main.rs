extern crate WinApp;
use WinApp::window::Window;

use std::mem;
use std::ptr;
use winapi::um::winuser::CW_USEDEFAULT;
use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, UINT, WPARAM},
        windef::{HBRUSH, HWND},
    },
    um::{
        wingdi::{GetStockObject, WHITE_BRUSH},
        winuser::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, LoadIconW,
            PostQuitMessage, RegisterClassW, ShowWindow, TranslateMessage, UpdateWindow,
            CS_HREDRAW, CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MSG, SW_NORMAL, WM_DESTROY,
            WNDCLASSW, WS_OVERLAPPEDWINDOW,
        },
    },
};

fn main() {
    unsafe {
        let class_name = encode("my_window_class_name");
        let window_name = encode("Hello, World!");

        let window = match Window::create(&class_name, &window_name) {
            Ok(result) => result,
            Err(message) => {
                println!("{}", message);
                return;
            }
        };
        window.show(SW_NORMAL);
        window.update();
        let mut msg = mem::uninitialized::<MSG>();
        loop {
            if GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                return;
            }
            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
    }
}

fn encode(source: &str) -> Vec<u16> {
    source.encode_utf16().chain(Some(0)).collect()
}
