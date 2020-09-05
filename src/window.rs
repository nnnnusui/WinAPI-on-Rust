use std::{error, fmt, mem, ptr};
use winapi::um::winuser::{DispatchMessageW, GetMessageW, TranslateMessage, MSG};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{BOOL, LPARAM, LRESULT, UINT, WPARAM},
        windef::{HBRUSH, HWND},
    },
    um::{
        wingdi::{GetStockObject, WHITE_BRUSH},
        winuser::{
            CreateWindowExW, DefWindowProcW, LoadCursorW, LoadIconW, PostQuitMessage,
            RegisterClassW, ShowWindow, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
            IDC_ARROW, IDI_APPLICATION, WM_DESTROY, WNDCLASSW, WS_OVERLAPPEDWINDOW,
        },
    },
};

pub struct Window {
    pub hwnd: HWND,
    msg: mem::MaybeUninit<MSG>,
}

impl Window {
    unsafe fn register_wndclass(class_name: &[u16]) -> bool {
        let mut winc = mem::zeroed::<WNDCLASSW>();
        winc.style = CS_HREDRAW | CS_VREDRAW;
        winc.lpfnWndProc = Some(Window::win_proc);
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

    pub unsafe fn create(class_name: &[u16], window_name: &[u16]) -> Result<Window, WindowError> {
        if !Window::register_wndclass(&class_name) {
            return Err(WindowError::RegisterClassFailed);
        }
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
            return Err(WindowError::CreateWindowFailed);
        }
        let msg = mem::MaybeUninit::<MSG>::uninit();
        Ok(Window { hwnd, msg })
    }
    pub unsafe fn show(&self, n_cmd_show: c_int) -> BOOL {
        ShowWindow(self.hwnd, n_cmd_show)
    }
    pub unsafe fn update(&self) -> BOOL {
        UpdateWindow(self.hwnd)
    }
    pub unsafe fn frame(&self) -> bool {
        let mut msg = self.msg.assume_init();
        if GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
            return false;
        }
        TranslateMessage(&mut msg);
        DispatchMessageW(&mut msg);
        true
    }
}

#[derive(Debug)]
pub enum WindowError {
    CreateWindowFailed,
    RegisterClassFailed,
}
impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WindowError::CreateWindowFailed => write!(f, "CreateWindow returned null"),
            WindowError::RegisterClassFailed => write!(f, "RegisterClass returned error"),
        }
    }
}

impl error::Error for WindowError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            WindowError::CreateWindowFailed => None,
            WindowError::RegisterClassFailed => None,
        }
    }
}
