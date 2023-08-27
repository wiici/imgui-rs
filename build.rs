use std::{
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from_str(env::var("OUT_DIR").unwrap().as_str())
        .expect("Cannot get OUT_DIR env var");

    let mut cmake_config = cmake::Config::new("libcimgui");
    if cfg!(feature = "dx11") {
        cmake_config.define("IMGUI_BUILD_WITH_DX11", "TRUE");
    }
    if cfg!(feature = "win32") {
        cmake_config.define("IMGUI_BUILD_WITH_WIN32", "TRUE");
    }

    _ = cmake_config.build();

    println!("cargo:rustc-link-lib=libcimgui");
    println!("cargo:rustc-link-search={}", out_dir.join("lib").display());

    if env::var("GENERATE_IMGUI_BINDINGS").is_ok() {
        let imgui_header_include_dir = out_dir.join("include");
        let src_dir = manifest_dir.join("src");

        generate_imgui_bindings(imgui_header_include_dir.as_path(), &src_dir);

        if cfg!(feature = "dx11") {
            generate_imgui_dx11_bindings(imgui_header_include_dir.as_path(), &src_dir);
        }
        if cfg!(feature = "win32") {
            generate_imgui_win32_bindings(imgui_header_include_dir.as_path(), &src_dir);
        }
    }
}

fn generate_imgui_bindings(imgui_headers_dir: &Path, output_dir: &Path) {
    let imgui_bindings = bindgen::Builder::default()
        .header("libcimgui/generator_output/cimgui.h")
        .clang_arg(format!("-I{}", imgui_headers_dir.display()))
        .generate()
        .expect("Couldn't generate bindings");

    imgui_bindings
        .write_to_file(output_dir.join("imgui_bindings.rs"))
        .expect("Couldn't write bindings");

    println!("Create bindings for imgui.h");
}

fn generate_imgui_win32_bindings(imgui_headers_dir: &Path, output_dir: &Path) {
    let win32_bindings = bindgen::Builder::default()
        .header("libcimgui/generator_output/cimgui_impl_win32.h")
        .allowlist_function("cImGui_ImplWin32_Init")
        .allowlist_function("cImGui_ImplWin32_InitForOpenGL")
        .allowlist_function("cImGui_ImplWin32_Shutdown")
        .allowlist_function("cImGui_ImplWin32_NewFrame")
        .allowlist_function("cImGui_ImplWin32_WndProcHandler")
        .allowlist_function("cImGui_ImplWin32_EnableDpiAwareness")
        .allowlist_function("cImGui_ImplWin32_GetDpiScaleForHwnd")
        .allowlist_function("cImGui_ImplWin32_GetDpiScaleForMonitor")
        .allowlist_function("cImGui_ImplWin32_EnableAlphaCompositing")
        .clang_arg(format!("-I{}", imgui_headers_dir.display()))
        .generate()
        .expect("Couldn't generate bindings");

    win32_bindings
        .write_to_file(output_dir.join("imgui_impl_win32_bindings.rs"))
        .expect("Couldn't write bindings");

    println!("Create bindings for imgui_impl_win32.h");
}

fn generate_imgui_dx11_bindings(imgui_headers_dir: &Path, output_dir: &Path) {
    let dx11_bindings = bindgen::Builder::default()
        .header("libcimgui/generator_output/cimgui_impl_dx11.h")
        .allowlist_function("cImGui_ImplDX11_Init")
        .allowlist_function("cImGui_ImplDX11_Shutdown")
        .allowlist_function("cImGui_ImplDX11_NewFrame")
        .allowlist_function("cImGui_ImplDX11_RenderDrawData")
        .allowlist_function("cImGui_ImplDX11_InvalidateDeviceObjects")
        .allowlist_function("cImGui_ImplDX11_CreateDeviceObjects")
        .clang_arg(format!("-I{}", imgui_headers_dir.display()))
        .generate()
        .expect("Couldn't generate bindings");

    dx11_bindings
        .write_to_file(output_dir.join("imgui_impl_dx11_bindings.rs"))
        .expect("Couldn't write bindings");

    println!("Create bindings for imgui_impl_dx11.h");
}
