# Find DirectX 11 SDK
#
# Output variables:
# DX11_SDK_INCLUDE_DIR
# DX11_SDK_STATIC

if(NOT WIN32)
    return()
endif()

find_package(WindowsKits REQUIRED)

message(STATUS "Trying to find DirectX 11 SDK...")

# Assume that headers are stored inside "um" directory
set(INCLUDE_DIR_CANDIDATE "${WINDOWS_KITS_INCLUDE_DIR}/um")
find_path(DX11_SDK_INCLUDE_DIR NAMES d3d11.h PATHS ${INCLUDE_DIR_CANDIDATE})

if (NOT DX11_SDK_INCLUDE_DIR)
    message(FATAL_ERROR "Failed to find DirectX 11 headers in \"${INCLUDE_DIR_CANDIDATE}\"")
endif()

if (CMAKE_SIZEOF_VOID_P EQUAL 8)
    set(ARCH_DIR_NAME "x64")
else()
    set(ARCH_DIR_NAME "x86")
endif()

# Assume that libraries are stored inside "um" directory
set(LIB_DIR_CANDIDATE "${WINDOWS_KITS_LIBS_DIR}/um/${ARCH_DIR_NAME}")
find_library(DX11_LIB NAMES d3d11.lib PATHS ${LIB_DIR_CANDIDATE})
find_library(DXGI_LIB NAMES dxgi.lib PATHS ${LIB_DIR_CANDIDATE})
set(DX11_SDK_STATIC ${DX11_LIB} ${DXGI_LIB})

if (NOT DX11_SDK_STATIC)
    message(FATAL_ERROR "Failed to find DirectX 11 libraries in \"${LIB_DIR_CANDIDATE}\"")
endif()

message(STATUS "Found DirectX 11 SDK:")
message(STATUS "Libs: \"${DX11_SDK_STATIC}\"")
message(STATUS "Headers: \"${DX11_SDK_INCLUDE_DIR}\"")
