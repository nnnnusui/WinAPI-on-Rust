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
        if !register_wndclass(&class_name) {
            return;
        }

        let window = Window::create(&class_name, &encode("Hello, World!"));
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

unsafe fn register_wndclass(class_name: &[u16]) -> bool {
    let mut winc = mem::zeroed::<WNDCLASSW>();
    winc.style = CS_HREDRAW | CS_VREDRAW;
    winc.lpfnWndProc = Some(win_proc);
    winc.hIcon = LoadIconW(ptr::null_mut(), IDI_APPLICATION);
    winc.hCursor = LoadCursorW(ptr::null_mut(), IDC_ARROW);
    winc.hbrBackground = GetStockObject(WHITE_BRUSH as i32) as HBRUSH;
    winc.lpszClassName = class_name.as_ptr();

    RegisterClassW(&winc) > 0
}

unsafe extern "system" fn win_proc(
    hwnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    };
    0
}
