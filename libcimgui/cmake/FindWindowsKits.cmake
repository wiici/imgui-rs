# Find Windows Kits

# Variables:
# NEWEST_WINDOWS_KITS_VER
# WINDOWS_KITS_DIR
# WINDOWS_KITS_INCLUDE_DIR - for the newest Windows Kits version
# WINDOWS_KITS_LIBS_DIR - for the newest Windows Kits version

if (NOT WIN32)
    return()
endif()

# Source: https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/
# Only latest 4 versions are checked
set(WK_VER_LOOKUP_TABLE 10.0.22621.0
                        10.0.22000.0
                        10.0.20348.0
                        10.0.19041.0)

message(STATUS "Trying to find Windows Kits...")

# Checking only this register might not be enough
get_filename_component(WK_DIR "[HKEY_LOCAL_MACHINE\\SOFTWARE\\WOW6432Node\\Microsoft\\Microsoft SDKs\\Windows\\v10.0;InstallationFolder]" ABSOLUTE)

if (WK_DIR)
    message(STATUS "Windows Kits directory found: \"${WK_DIR}\"")
else()
    message(FATAL_ERROR "Failed to find Windows Kits directory")
endif()

foreach(CURR_WK_VER ${WK_VER_LOOKUP_TABLE})
    set(PATH_TO_CHECK ${WK_DIR}/Include/${CURR_WK_VER})
    message(VERBOSE "Checking Include dir under \"${PATH_TO_CHECK}\"")

    if (EXISTS ${PATH_TO_CHECK})
        set(NEWEST_WINDOWS_KITS_VER ${CURR_WK_VER})
        set(WINDOWS_KITS_INCLUDE_DIR ${PATH_TO_CHECK})
        break()
    endif()
endforeach()

if (NEWEST_WINDOWS_KITS_VER)
    message(STATUS "Found newest availalbe Windows Kits version: ${NEWEST_WINDOWS_KITS_VER}")
else()
    message(FATAL_ERROR "Failed to find newest availalbe Windows Kits version")
endif()

if (WINDOWS_KITS_INCLUDE_DIR)
    message(STATUS "Found Include directory in Windows Kits folder: \"${WINDOWS_KITS_INCLUDE_DIR}\"")
else()
    message(FATAL_ERROR "Failed to find Include directory in Windows Kits folder for version ${NEWEST_WINDOWS_KITS_VER}")
endif()

set(PATH_TO_CHECK ${WK_DIR}/Lib/${NEWEST_WINDOWS_KITS_VER})
if (EXISTS ${PATH_TO_CHECK})
    set(WINDOWS_KITS_LIBS_DIR ${PATH_TO_CHECK})
    message(STATUS "Found Lib directory in Windows Kits folder: \"${WINDOWS_KITS_LIBS_DIR}\"")
else()
    message(FATAL_ERROR "Failed to find Lib directory in Windows Kits folder for version ${NEWEST_WINDOWS_KITS_VER}")
endif()
