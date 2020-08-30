use winapi::{
    um::{
        winuser::{RegisterClassW, WNDCLASSW, CS_HREDRAW, CS_VREDRAW,
                  LoadIconW, IDI_APPLICATION, LoadCursorW, IDC_ARROW,
                  CreateWindowExW, ShowWindow, SW_NORMAL, UpdateWindow,
                  GetMessageW, TranslateMessage, DispatchMessageW, MSG,
                  WM_DESTROY, PostQuitMessage, DefWindowProcW, WS_OVERLAPPEDWINDOW},
        wingdi::{GetStockObject, WHITE_BRUSH},
    },
    shared::{
        windef::{HWND, HBRUSH},
        minwindef::{UINT, WPARAM, LPARAM, LRESULT},
    },
};
use std::ptr;
use std::mem;
use winapi::um::winuser::CW_USEDEFAULT;

fn main() {
    unsafe {
        let class_name = encode("my_window_class_name");
        if !register_wndclass(&class_name) {
            return;
        }

        let hwnd = create_window(&class_name);
        if hwnd.is_null() {
            return;
        }
        ShowWindow(hwnd, SW_NORMAL);
        UpdateWindow(hwnd);
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
unsafe fn create_window(class_name: &[u16]) -> HWND {
    CreateWindowExW(
        0, //dwExStyle: DWORD
        class_name.as_ptr(), //lpClassName: LPCWSTR
        encode("Hello, World!").as_ptr(), //lpWindowName: LPCWSTR
        WS_OVERLAPPEDWINDOW, //dwStyle: DWORD
        CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, //X: int, Y: int, nWidth: int, nHeight: int
        ptr::null_mut(), //hWndParent: HWND
        ptr::null_mut(), //hMenu: HMENU
        ptr::null_mut(), //hInstance: HINSTANCE
        ptr::null_mut(), //lpParam: LPVOID
    )
}
unsafe extern "system" fn win_proc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    };
    0
}
