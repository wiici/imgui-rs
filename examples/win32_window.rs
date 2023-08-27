use windows::core::{Error, PCWSTR};
use windows::Win32::Foundation::{
    HMODULE, HWND,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DestroyWindow, RegisterClassW, UnregisterClassW, CS_VREDRAW, CW_USEDEFAULT, HMENU, WINDOW_EX_STYLE,
    WNDCLASSW, WNDPROC, WS_TILEDWINDOW, WS_VISIBLE,
};

#[derive(Default)]
pub struct Window {
    pub hwnd: HWND,
    utf16_class_name: Vec<u16>,
    hinstance: HMODULE,
}

impl Window {
    pub fn new(window_class_name: &str, window_proc: WNDPROC) -> Result<Window, String> {
        let hinstance = { unsafe { GetModuleHandleW(PCWSTR::null()).unwrap() } };

        let utf16_class_name = window_class_name
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>();

        let window_class = WNDCLASSW {
            style: CS_VREDRAW,
            lpfnWndProc: window_proc,
            hInstance: hinstance,
            lpszClassName: PCWSTR(utf16_class_name.as_ptr()),

            ..Default::default()
        };

        let register_result = { unsafe { RegisterClassW(&window_class) } };
        if register_result != 0 {
            println!("Register window class");
        } else {
            return Err(format!("RegisterClassW failed: {}", Error::from_win32()));
        }

        let hwnd = {
            unsafe {
                CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    PCWSTR(utf16_class_name.as_ptr()),
                    PCWSTR::null(),
                    WS_VISIBLE | WS_TILEDWINDOW,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    HWND::default(),
                    HMENU::default(),
                    hinstance,
                    None,
                )
            }
        };

        if hwnd == HWND::default() {
            return Err(format!("CreateWindowExW failed: {}", Error::from_win32()));
        }

        Ok(Window {
            hwnd,
            utf16_class_name,
            hinstance,
        })
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.hwnd);
            UnregisterClassW(PCWSTR(self.utf16_class_name.as_ptr()), self.hinstance);
        }

        println!("Drop Window object")
    }
}
