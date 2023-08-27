pub use windows::core::{ComInterface, Error, IUnknown, Interface, HSTRING, PCWSTR};
use windows::Win32::Foundation::{HMODULE, HWND, TRUE};
pub use windows::Win32::Graphics::Direct3D::{
    D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1,
};
pub use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDeviceAndSwapChain, ID3D11DepthStencilView, ID3D11Device, ID3D11Device5,
    ID3D11DeviceContext, ID3D11DeviceContext4, ID3D11RenderTargetView, ID3D11RenderTargetView1,
    ID3D11Resource, ID3D11Texture2D, D3D11_CREATE_DEVICE_DEBUG, D3D11_CREATE_DEVICE_FLAG,
    D3D11_SDK_VERSION,
};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC, DXGI_RATIONAL, DXGI_SAMPLE_DESC,
};
pub use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory, CreateDXGIFactory1, CreateDXGIFactory2, IDXGIAdapter, IDXGIAdapter2,
    IDXGIAdapter4, IDXGIDevice4, IDXGIFactory7, IDXGIOutput6, IDXGISwapChain, DXGI_ADAPTER_DESC,
    DXGI_ADAPTER_DESC2, DXGI_ADAPTER_DESC3, DXGI_CREATE_FACTORY_DEBUG, DXGI_ERROR_NOT_FOUND,
    DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE, DXGI_SWAP_CHAIN_DESC,
    DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_EFFECT_FLIP_DISCARD,
    DXGI_USAGE_RENDER_TARGET_OUTPUT,
};

pub struct Renderer {
    pub device: ID3D11Device5,
    pub device_context: ID3D11DeviceContext4,
    pub swap_chain: IDXGISwapChain,
    pub main_rtv: ID3D11RenderTargetView,
}

impl Renderer {
    pub fn new(hwnd: HWND) -> Result<Renderer, Error> {
        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
            BufferCount: 3,
            BufferDesc: DXGI_MODE_DESC {
                Width: 0,
                Height: 0,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                RefreshRate: DXGI_RATIONAL {
                    Numerator: 60,
                    Denominator: 1,
                },
                ..Default::default()
            },
            Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0 as u32,
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            OutputWindow: hwnd,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Windowed: TRUE,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,

            ..Default::default()
        };

        let dxgi_factory_flags = {
            let mut flags = 0u32;
            if cfg!(debug_assertions) {
                flags |= DXGI_CREATE_FACTORY_DEBUG;
            }

            flags
        };

        // let dxgi_adapter = unsafe { CreateDXGIFactory::<IDXGIAdapter2>(dxgi_factory_flags) }?;
        let dxgi_factory = unsafe { CreateDXGIFactory::<IDXGIFactory7>() }.unwrap();
        let mut adapter_idx = 0;
        loop {
            if let Ok(adapter) = unsafe {
                dxgi_factory.EnumAdapterByGpuPreference::<IDXGIAdapter4>(
                    adapter_idx,
                    DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
                )
            } {
                let mut adapter_desc = DXGI_ADAPTER_DESC3::default();
                unsafe { adapter.GetDesc3(&mut adapter_desc) };
                // info!(
                //     "DXGI Adapter {}: {}",
                //     adapter_idx,
                //     String::from_utf16(&adapter_desc.Description).unwrap()
                // );
            } else {
                break;
            }

            adapter_idx += 1;
        }

        let mut adapter_desc = DXGI_ADAPTER_DESC3::default();
        let dxgi_adapter = unsafe {
            dxgi_factory.EnumAdapterByGpuPreference::<IDXGIAdapter4>(
                0,
                DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
            )
        }
        .unwrap();

        unsafe { dxgi_adapter.GetDesc3(&mut adapter_desc) }.unwrap();

        // info!(
        //     "Choose default dxgi adapter:
        //     Device id: {}
        //     Device description: {}",
        //     adapter_desc.DeviceId,
        //     String::from_utf16(&adapter_desc.Description).unwrap()
        // );

        let mut feature_level = D3D_FEATURE_LEVEL::default();
        let device_creation_flags = {
            let mut flags = D3D11_CREATE_DEVICE_FLAG::default();
            if cfg!(debug_assertions) {
                flags |= D3D11_CREATE_DEVICE_DEBUG;
            }

            flags
        };
        let feature_levels_array = [D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1];

        let mut swap_chain_output: Option<IDXGISwapChain> = None;
        let mut device_output: Option<ID3D11Device> = None;
        let mut device_context_output: Option<ID3D11DeviceContext> = None;
        let result = unsafe {
            D3D11CreateDeviceAndSwapChain(
                &dxgi_adapter,
                D3D_DRIVER_TYPE_UNKNOWN,
                HMODULE::default(),
                device_creation_flags,
                Some(&feature_levels_array),
                D3D11_SDK_VERSION,
                Some(&swap_chain_desc),
                Some(&mut swap_chain_output),
                Some(&mut device_output),
                None,
                Some(&mut device_context_output),
            )
        }?;
        // if let Err(err) = result {
        //     return Err!(format!(
        //         "D3D11CreateDeviceAndSwapChain failed with error: {}",
        //         err
        //     ));
        // }

        let swap_chain = swap_chain_output.unwrap();
        let device = device_output.unwrap();
        let device_context = device_context_output.unwrap();

        let back_buffer = unsafe { swap_chain.GetBuffer::<ID3D11Texture2D>(0) }?;
        let mut rtv_output: Option<ID3D11RenderTargetView> = None;
        unsafe { device.CreateRenderTargetView(&back_buffer, None, Some(&mut rtv_output)) }?;
        let rtv = rtv_output.unwrap();

        Ok(Renderer {
            device: device.cast::<ID3D11Device5>().unwrap(),
            device_context: device_context.cast::<ID3D11DeviceContext4>().unwrap(),
            swap_chain: swap_chain,
            main_rtv: rtv,
        })
    }
}
