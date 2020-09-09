extern crate winapp;
use winapp::window::Window;

use std::mem;
use std::ptr;
use winapi::um::winuser::SW_NORMAL;

fn main() {
    unsafe { show_window() }
}

unsafe fn show_window() {
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
    while window.frame() {}
}

fn encode(source: &str) -> Vec<u16> {
    source.encode_utf16().chain(Some(0)).collect()
}
