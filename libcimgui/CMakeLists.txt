cmake_minimum_required(VERSION 3.2)

project(libcimgui)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake)

include(FetchContent)
FetchContent_Declare(
    imgui
    GIT_REPOSITORY https://github.com/ocornut/imgui.git
    GIT_TAG        63b6300c39c97c2e87800b73f4d8abd3b8a1df94 # docking 1.89.9
)
FetchContent_MakeAvailable(imgui)

add_library(${PROJECT_NAME} STATIC)
target_sources(
    ${PROJECT_NAME}
    PRIVATE
    ${imgui_SOURCE_DIR}/imgui.cpp
    ${imgui_SOURCE_DIR}/imgui_demo.cpp
    ${imgui_SOURCE_DIR}/imgui_draw.cpp
    ${imgui_SOURCE_DIR}/imgui_tables.cpp
    ${imgui_SOURCE_DIR}/imgui_widgets.cpp
    ${imgui_SOURCE_DIR}/misc/cpp/imgui_stdlib.cpp
    ${PROJECT_SOURCE_DIR}/generator_output/cimgui.cpp
)
target_include_directories(
    ${PROJECT_NAME}
    PRIVATE
    ${imgui_SOURCE_DIR}
    ${imgui_SOURCE_DIR}/backends
)

if(IMGUI_BUILD_WITH_DX11)
    target_sources(
        ${PROJECT_NAME}
        PRIVATE
        ${imgui_SOURCE_DIR}/backends/imgui_impl_dx11.cpp
        ${PROJECT_SOURCE_DIR}/generator_output/cimgui_impl_dx11.cpp
    )
endif()

if(IMGUI_BUILD_WITH_WIN32)
    target_sources(
        ${PROJECT_NAME}
        PRIVATE
        ${imgui_SOURCE_DIR}/backends/imgui_impl_win32.cpp
        ${PROJECT_SOURCE_DIR}/generator_output/cimgui_impl_win32.cpp
    )
endif()

if(IMGUI_BUILD_WITH_DX11)
    find_package(DX11 REQUIRED)
    target_link_libraries(
        ${PROJECT_NAME}
        PRIVATE
        ${DX11_SDK_STATIC}
    )
endif()

install(
    TARGETS ${PROJECT_NAME}
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
)

install(
    FILES
    ${imgui_SOURCE_DIR}/imgui.h
    ${imgui_SOURCE_DIR}/imconfig.h
    ${imgui_SOURCE_DIR}/imgui_internal.h
    ${imgui_SOURCE_DIR}/imstb_textedit.h
    ${imgui_SOURCE_DIR}/imstb_rectpack.h
    ${imgui_SOURCE_DIR}/imstb_truetype.h
    ${imgui_SOURCE_DIR}/misc/cpp/imgui_stdlib.h
    DESTINATION include
)

if(IMGUI_BUILD_WITH_DX11)
    install(
        FILES
        ${imgui_SOURCE_DIR}/backends/imgui_impl_dx11.h
        DESTINATION include
    )
endif()

if(IMGUI_BUILD_WITH_WIN32)
    install(
        FILES ${imgui_SOURCE_DIR}/backends/imgui_impl_win32.h
        DESTINATION include
    )
endif()
