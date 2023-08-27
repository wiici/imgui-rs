/* automatically generated by rust-bindgen 0.66.1 */

pub type ImVec2 = ImVec2_t;
pub type ImVec4 = ImVec4_t;
pub type ImVector_ImDrawCmd = ImVector_ImDrawCmd_t;
pub type ImVector_ImDrawIdx = ImVector_ImDrawIdx_t;
pub type ImVector_ImDrawChannel = ImVector_ImDrawChannel_t;
pub type ImVector_ImDrawVert = ImVector_ImDrawVert_t;
pub type ImVector_ImVec4 = ImVector_ImVec4_t;
pub type ImVector_ImTextureID = ImVector_ImTextureID_t;
pub type ImVector_ImVec2 = ImVector_ImVec2_t;
pub type ImVector_ImDrawListPtr = ImVector_ImDrawListPtr_t;
pub type ImDrawCmdHeader = ImDrawCmdHeader_t;
pub type ImDrawChannel = ImDrawChannel_t;
pub type ImDrawCmd = ImDrawCmd_t;
pub type ImDrawData = ImDrawData_t;
pub type ImDrawList = ImDrawList_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImDrawListSharedData_t {
    _unused: [u8; 0],
}
pub type ImDrawListSharedData = ImDrawListSharedData_t;
pub type ImDrawListSplitter = ImDrawListSplitter_t;
pub type ImDrawVert = ImDrawVert_t;
pub type ImGuiViewport = ImGuiViewport_t;
pub type ImDrawListFlags = ::std::os::raw::c_int;
pub type ImGuiViewportFlags = ::std::os::raw::c_int;
pub type ImTextureID = *mut ::std::os::raw::c_void;
pub type ImDrawIdx = ::std::os::raw::c_ushort;
pub type ImGuiID = ::std::os::raw::c_uint;
pub type ImU32 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVec2_t {
    pub x: f32,
    pub y: f32,
}
#[test]
fn bindgen_test_layout_ImVec2_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImVec2_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImVec2_t>(),
        8usize,
        concat!("Size of: ", stringify!(ImVec2_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVec2_t>(),
        4usize,
        concat!("Alignment of ", stringify!(ImVec2_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVec2_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVec2_t),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVec4_t {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[test]
fn bindgen_test_layout_ImVec4_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImVec4_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImVec4_t>(),
        16usize,
        concat!("Size of: ", stringify!(ImVec4_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVec4_t>(),
        4usize,
        concat!("Alignment of ", stringify!(ImVec4_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVec4_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVec4_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVec4_t),
            "::",
            stringify!(z)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).w) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVec4_t),
            "::",
            stringify!(w)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVector_ImDrawCmd_t {
    pub Size: ::std::os::raw::c_int,
    pub Capacity: ::std::os::raw::c_int,
    pub Data: *mut ImDrawCmd,
}
#[test]
fn bindgen_test_layout_ImVector_ImDrawCmd_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImVector_ImDrawCmd_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImVector_ImDrawCmd_t>(),
        16usize,
        concat!("Size of: ", stringify!(ImVector_ImDrawCmd_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVector_ImDrawCmd_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImVector_ImDrawCmd_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawCmd_t),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Capacity) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawCmd_t),
            "::",
            stringify!(Capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawCmd_t),
            "::",
            stringify!(Data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVector_ImDrawIdx_t {
    pub Size: ::std::os::raw::c_int,
    pub Capacity: ::std::os::raw::c_int,
    pub Data: *mut ImDrawIdx,
}
#[test]
fn bindgen_test_layout_ImVector_ImDrawIdx_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImVector_ImDrawIdx_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImVector_ImDrawIdx_t>(),
        16usize,
        concat!("Size of: ", stringify!(ImVector_ImDrawIdx_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVector_ImDrawIdx_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImVector_ImDrawIdx_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawIdx_t),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Capacity) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawIdx_t),
            "::",
            stringify!(Capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawIdx_t),
            "::",
            stringify!(Data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVector_ImDrawChannel_t {
    pub Size: ::std::os::raw::c_int,
    pub Capacity: ::std::os::raw::c_int,
    pub Data: *mut ImDrawChannel,
}
#[test]
fn bindgen_test_layout_ImVector_ImDrawChannel_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImVector_ImDrawChannel_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImVector_ImDrawChannel_t>(),
        16usize,
        concat!("Size of: ", stringify!(ImVector_ImDrawChannel_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVector_ImDrawChannel_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImVector_ImDrawChannel_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawChannel_t),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Capacity) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawChannel_t),
            "::",
            stringify!(Capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawChannel_t),
            "::",
            stringify!(Data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVector_ImDrawVert_t {
    pub Size: ::std::os::raw::c_int,
    pub Capacity: ::std::os::raw::c_int,
    pub Data: *mut ImDrawVert,
}
#[test]
fn bindgen_test_layout_ImVector_ImDrawVert_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImVector_ImDrawVert_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImVector_ImDrawVert_t>(),
        16usize,
        concat!("Size of: ", stringify!(ImVector_ImDrawVert_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVector_ImDrawVert_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImVector_ImDrawVert_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawVert_t),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Capacity) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawVert_t),
            "::",
            stringify!(Capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawVert_t),
            "::",
            stringify!(Data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVector_ImVec4_t {
    pub Size: ::std::os::raw::c_int,
    pub Capacity: ::std::os::raw::c_int,
    pub Data: *mut ImVec4,
}
#[test]
fn bindgen_test_layout_ImVector_ImVec4_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImVector_ImVec4_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImVector_ImVec4_t>(),
        16usize,
        concat!("Size of: ", stringify!(ImVector_ImVec4_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVector_ImVec4_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImVector_ImVec4_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImVec4_t),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Capacity) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImVec4_t),
            "::",
            stringify!(Capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImVec4_t),
            "::",
            stringify!(Data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVector_ImTextureID_t {
    pub Size: ::std::os::raw::c_int,
    pub Capacity: ::std::os::raw::c_int,
    pub Data: *mut ImTextureID,
}
#[test]
fn bindgen_test_layout_ImVector_ImTextureID_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImVector_ImTextureID_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImVector_ImTextureID_t>(),
        16usize,
        concat!("Size of: ", stringify!(ImVector_ImTextureID_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVector_ImTextureID_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImVector_ImTextureID_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImTextureID_t),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Capacity) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImTextureID_t),
            "::",
            stringify!(Capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImTextureID_t),
            "::",
            stringify!(Data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVector_ImVec2_t {
    pub Size: ::std::os::raw::c_int,
    pub Capacity: ::std::os::raw::c_int,
    pub Data: *mut ImVec2,
}
#[test]
fn bindgen_test_layout_ImVector_ImVec2_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImVector_ImVec2_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImVector_ImVec2_t>(),
        16usize,
        concat!("Size of: ", stringify!(ImVector_ImVec2_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVector_ImVec2_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImVector_ImVec2_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImVec2_t),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Capacity) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImVec2_t),
            "::",
            stringify!(Capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImVec2_t),
            "::",
            stringify!(Data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVector_ImDrawListPtr_t {
    pub Size: ::std::os::raw::c_int,
    pub Capacity: ::std::os::raw::c_int,
    pub Data: *mut *mut ImDrawList,
}
#[test]
fn bindgen_test_layout_ImVector_ImDrawListPtr_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImVector_ImDrawListPtr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImVector_ImDrawListPtr_t>(),
        16usize,
        concat!("Size of: ", stringify!(ImVector_ImDrawListPtr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVector_ImDrawListPtr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImVector_ImDrawListPtr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawListPtr_t),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Capacity) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawListPtr_t),
            "::",
            stringify!(Capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImVector_ImDrawListPtr_t),
            "::",
            stringify!(Data)
        )
    );
}
pub type ImDrawCallback = ::std::option::Option<
    unsafe extern "C" fn(parent_list: *const ImDrawList, cmd: *const ImDrawCmd),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImDrawCmd_t {
    pub ClipRect: ImVec4,
    pub TextureId: ImTextureID,
    pub VtxOffset: ::std::os::raw::c_uint,
    pub IdxOffset: ::std::os::raw::c_uint,
    pub ElemCount: ::std::os::raw::c_uint,
    pub UserCallback: ImDrawCallback,
    pub UserCallbackData: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_ImDrawCmd_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImDrawCmd_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImDrawCmd_t>(),
        56usize,
        concat!("Size of: ", stringify!(ImDrawCmd_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImDrawCmd_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImDrawCmd_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ClipRect) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawCmd_t),
            "::",
            stringify!(ClipRect)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TextureId) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawCmd_t),
            "::",
            stringify!(TextureId)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).VtxOffset) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawCmd_t),
            "::",
            stringify!(VtxOffset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IdxOffset) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawCmd_t),
            "::",
            stringify!(IdxOffset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ElemCount) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawCmd_t),
            "::",
            stringify!(ElemCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).UserCallback) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawCmd_t),
            "::",
            stringify!(UserCallback)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).UserCallbackData) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawCmd_t),
            "::",
            stringify!(UserCallbackData)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImDrawVert_t {
    pub pos: ImVec2,
    pub uv: ImVec2,
    pub col: ImU32,
}
#[test]
fn bindgen_test_layout_ImDrawVert_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImDrawVert_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImDrawVert_t>(),
        20usize,
        concat!("Size of: ", stringify!(ImDrawVert_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImDrawVert_t>(),
        4usize,
        concat!("Alignment of ", stringify!(ImDrawVert_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pos) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawVert_t),
            "::",
            stringify!(pos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uv) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawVert_t),
            "::",
            stringify!(uv)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).col) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawVert_t),
            "::",
            stringify!(col)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImDrawCmdHeader_t {
    pub ClipRect: ImVec4,
    pub TextureId: ImTextureID,
    pub VtxOffset: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_ImDrawCmdHeader_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImDrawCmdHeader_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImDrawCmdHeader_t>(),
        32usize,
        concat!("Size of: ", stringify!(ImDrawCmdHeader_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImDrawCmdHeader_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImDrawCmdHeader_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ClipRect) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawCmdHeader_t),
            "::",
            stringify!(ClipRect)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TextureId) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawCmdHeader_t),
            "::",
            stringify!(TextureId)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).VtxOffset) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawCmdHeader_t),
            "::",
            stringify!(VtxOffset)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImDrawChannel_t {
    pub _CmdBuffer: ImVector_ImDrawCmd,
    pub _IdxBuffer: ImVector_ImDrawIdx,
}
#[test]
fn bindgen_test_layout_ImDrawChannel_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImDrawChannel_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImDrawChannel_t>(),
        32usize,
        concat!("Size of: ", stringify!(ImDrawChannel_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImDrawChannel_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImDrawChannel_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._CmdBuffer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawChannel_t),
            "::",
            stringify!(_CmdBuffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IdxBuffer) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawChannel_t),
            "::",
            stringify!(_IdxBuffer)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImDrawListSplitter_t {
    pub _Current: ::std::os::raw::c_int,
    pub _Count: ::std::os::raw::c_int,
    pub _Channels: ImVector_ImDrawChannel,
}
#[test]
fn bindgen_test_layout_ImDrawListSplitter_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImDrawListSplitter_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImDrawListSplitter_t>(),
        24usize,
        concat!("Size of: ", stringify!(ImDrawListSplitter_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImDrawListSplitter_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImDrawListSplitter_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Current) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawListSplitter_t),
            "::",
            stringify!(_Current)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Count) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawListSplitter_t),
            "::",
            stringify!(_Count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Channels) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawListSplitter_t),
            "::",
            stringify!(_Channels)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImDrawList_t {
    pub CmdBuffer: ImVector_ImDrawCmd,
    pub IdxBuffer: ImVector_ImDrawIdx,
    pub VtxBuffer: ImVector_ImDrawVert,
    pub Flags: ImDrawListFlags,
    pub _VtxCurrentIdx: ::std::os::raw::c_uint,
    pub _Data: *mut ImDrawListSharedData,
    pub _OwnerName: *const ::std::os::raw::c_char,
    pub _VtxWritePtr: *mut ImDrawVert,
    pub _IdxWritePtr: *mut ImDrawIdx,
    pub _ClipRectStack: ImVector_ImVec4,
    pub _TextureIdStack: ImVector_ImTextureID,
    pub _Path: ImVector_ImVec2,
    pub _CmdHeader: ImDrawCmdHeader,
    pub _Splitter: ImDrawListSplitter,
    pub _FringeScale: f32,
}
#[test]
fn bindgen_test_layout_ImDrawList_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImDrawList_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImDrawList_t>(),
        200usize,
        concat!("Size of: ", stringify!(ImDrawList_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImDrawList_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImDrawList_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).CmdBuffer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(CmdBuffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IdxBuffer) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(IdxBuffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).VtxBuffer) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(VtxBuffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Flags) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._VtxCurrentIdx) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_VtxCurrentIdx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Data) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_Data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._OwnerName) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_OwnerName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._VtxWritePtr) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_VtxWritePtr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IdxWritePtr) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_IdxWritePtr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._ClipRectStack) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_ClipRectStack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._TextureIdStack) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_TextureIdStack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Path) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_Path)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._CmdHeader) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_CmdHeader)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Splitter) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_Splitter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._FringeScale) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawList_t),
            "::",
            stringify!(_FringeScale)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImDrawData_t {
    pub Valid: bool,
    pub CmdListsCount: ::std::os::raw::c_int,
    pub TotalIdxCount: ::std::os::raw::c_int,
    pub TotalVtxCount: ::std::os::raw::c_int,
    pub CmdLists: ImVector_ImDrawListPtr,
    pub DisplayPos: ImVec2,
    pub DisplaySize: ImVec2,
    pub FramebufferScale: ImVec2,
    pub OwnerViewport: *mut ImGuiViewport,
}
#[test]
fn bindgen_test_layout_ImDrawData_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImDrawData_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImDrawData_t>(),
        64usize,
        concat!("Size of: ", stringify!(ImDrawData_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImDrawData_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImDrawData_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Valid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawData_t),
            "::",
            stringify!(Valid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).CmdListsCount) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawData_t),
            "::",
            stringify!(CmdListsCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TotalIdxCount) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawData_t),
            "::",
            stringify!(TotalIdxCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TotalVtxCount) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawData_t),
            "::",
            stringify!(TotalVtxCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).CmdLists) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawData_t),
            "::",
            stringify!(CmdLists)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DisplayPos) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawData_t),
            "::",
            stringify!(DisplayPos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DisplaySize) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawData_t),
            "::",
            stringify!(DisplaySize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).FramebufferScale) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawData_t),
            "::",
            stringify!(FramebufferScale)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).OwnerViewport) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ImDrawData_t),
            "::",
            stringify!(OwnerViewport)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImGuiViewport_t {
    pub ID: ImGuiID,
    pub Flags: ImGuiViewportFlags,
    pub Pos: ImVec2,
    pub Size: ImVec2,
    pub WorkPos: ImVec2,
    pub WorkSize: ImVec2,
    pub DpiScale: f32,
    pub ParentViewportId: ImGuiID,
    pub DrawData: *mut ImDrawData,
    pub RendererUserData: *mut ::std::os::raw::c_void,
    pub PlatformUserData: *mut ::std::os::raw::c_void,
    pub PlatformHandle: *mut ::std::os::raw::c_void,
    pub PlatformHandleRaw: *mut ::std::os::raw::c_void,
    pub PlatformWindowCreated: bool,
    pub PlatformRequestMove: bool,
    pub PlatformRequestResize: bool,
    pub PlatformRequestClose: bool,
}
#[test]
fn bindgen_test_layout_ImGuiViewport_t() {
    const UNINIT: ::std::mem::MaybeUninit<ImGuiViewport_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ImGuiViewport_t>(),
        96usize,
        concat!("Size of: ", stringify!(ImGuiViewport_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ImGuiViewport_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ImGuiViewport_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ID) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(ID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Flags) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Pos) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(Pos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).WorkPos) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(WorkPos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).WorkSize) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(WorkSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DpiScale) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(DpiScale)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ParentViewportId) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(ParentViewportId)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DrawData) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(DrawData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RendererUserData) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(RendererUserData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PlatformUserData) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(PlatformUserData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PlatformHandle) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(PlatformHandle)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PlatformHandleRaw) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(PlatformHandleRaw)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PlatformWindowCreated) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(PlatformWindowCreated)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PlatformRequestMove) as usize - ptr as usize },
        89usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(PlatformRequestMove)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PlatformRequestResize) as usize - ptr as usize },
        90usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(PlatformRequestResize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PlatformRequestClose) as usize - ptr as usize },
        91usize,
        concat!(
            "Offset of field: ",
            stringify!(ImGuiViewport_t),
            "::",
            stringify!(PlatformRequestClose)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ID3D11Device_t {
    _unused: [u8; 0],
}
pub type ID3D11Device = ID3D11Device_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ID3D11DeviceContext_t {
    _unused: [u8; 0],
}
pub type ID3D11DeviceContext = ID3D11DeviceContext_t;
extern "C" {
    pub fn cImGui_ImplDX11_Init(
        device: *mut ID3D11Device,
        device_context: *mut ID3D11DeviceContext,
    ) -> bool;
}
extern "C" {
    pub fn cImGui_ImplDX11_Shutdown();
}
extern "C" {
    pub fn cImGui_ImplDX11_NewFrame();
}
extern "C" {
    pub fn cImGui_ImplDX11_RenderDrawData(draw_data: *mut ImDrawData);
}
extern "C" {
    pub fn cImGui_ImplDX11_InvalidateDeviceObjects();
}
extern "C" {
    pub fn cImGui_ImplDX11_CreateDeviceObjects() -> bool;
}