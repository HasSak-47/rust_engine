#[cfg(target_os="windows")]
use ash::extensions::khr::Win32Surface;

#[cfg(
    all(
        unix,
        not(target_os="macos"),
        not(target_os="android")
    )
)]
use ash::extensions::khr::XlibSurface;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}
