use std::ffi::c_void;
use std::ffi::CStr;
use std::ops::BitAnd;

mod dx11_renderer;
mod win32_window;
use crate::dx11_renderer::*;
use crate::win32_window::*;
use imgui::ImGuiConfigFlags__ImGuiConfigFlags_ViewportsEnable;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

use windows::Win32::UI::WindowsAndMessaging::PM_REMOVE;
use windows::Win32::UI::WindowsAndMessaging::WM_DESTROY;
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, DispatchMessageW, PeekMessageW, PostQuitMessage, TranslateMessage, MSG,
    WM_QUIT, WNDPROC,
};
use windows::Win32::{
    Graphics::Gdi::UpdateWindow,
    UI::WindowsAndMessaging::{ShowWindow, SW_SHOWDEFAULT},
};

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    umsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let imgui_wndproc_result = imgui::cImGui_ImplWin32_WndProcHandler(
        hwnd.0 as imgui::HWND,
        umsg,
        wparam.0 as ::std::os::raw::c_ulonglong,
        lparam.0 as ::std::os::raw::c_longlong,
    );
    if imgui_wndproc_result != 0 {
        return LRESULT(1);
    }

    match umsg {
        WM_DESTROY => {
            println!("PostQuitMessage");
            PostQuitMessage(0);
        }
        _ => return DefWindowProcW(hwnd, umsg, wparam, lparam),
    }

    LRESULT(1)
}

fn main() {
    unsafe {
        let window = Window::new("ImGui win32+dx11", Some(wnd_proc)).unwrap();
        let renderer = Renderer::new(window.hwnd).unwrap();

        ShowWindow(window.hwnd, SW_SHOWDEFAULT);
        UpdateWindow(window.hwnd);

        let version = imgui::ImGui_GetVersion();
        println!("ImGui version: {:?}", CStr::from_ptr(version));

        let mut imgui_context = imgui::ImGui_CreateContext(std::ptr::null_mut());
        let mut imgui_io = imgui::ImGui_GetIO();

        (*imgui_io).ConfigFlags |= imgui::ImGuiConfigFlags__ImGuiConfigFlags_DockingEnable;
        (*imgui_io).ConfigFlags |= imgui::ImGuiConfigFlags__ImGuiConfigFlags_ViewportsEnable;
        println!("Create ImGui context");

        imgui::cImGui_ImplWin32_Init(window.hwnd.clone().0 as *mut c_void);
        imgui::cImGui_ImplDX11_Init(
            renderer.device.as_raw() as *mut imgui::ID3D11Device,
            renderer.device_context.as_raw() as *mut imgui::ID3D11DeviceContext,
        );

        'application_loop: loop {
            let mut msg = MSG::default();
            'handle_messages_loop: loop {
                let peek_msg_result = PeekMessageW(&mut msg, HWND::default(), 0, 0, PM_REMOVE);
                if !peek_msg_result.as_bool() {
                    break 'handle_messages_loop;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
                if msg.message == WM_QUIT {
                    println!("msg.message == WM_QUIT");
                    break 'application_loop;
                }
            }

            // Start the Dear ImGui frame
            imgui::cImGui_ImplDX11_NewFrame();
            imgui::cImGui_ImplWin32_NewFrame();
            imgui::ImGui_NewFrame();

            imgui::ImGui_DockSpaceOverViewport();

            let mut show_demo = true;
            imgui::ImGui_ShowDemoWindow(&mut show_demo);

            imgui::ImGui_Render();

            renderer
                .device_context
                .OMSetRenderTargets(Some(&[Some(renderer.main_rtv.clone())]), None);

            imgui::cImGui_ImplDX11_RenderDrawData(
                imgui::ImGui_GetDrawData() as *mut imgui::imgui_impl_dx11_bindings::ImDrawData_t
            );

            if (*imgui_io)
                .ConfigFlags
                .bitand(ImGuiConfigFlags__ImGuiConfigFlags_ViewportsEnable)
                != 0
            {
                imgui::ImGui_UpdatePlatformWindows();
                imgui::ImGui_RenderPlatformWindowsDefault();
            }

            renderer.swap_chain.Present(1, 0);
        } // End of application loop

        imgui::cImGui_ImplDX11_Shutdown();
        imgui::cImGui_ImplWin32_Shutdown();
        imgui::ImGui_DestroyContext(imgui_context);
    }
}
