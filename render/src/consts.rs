
use std::ffi::CStr;
use std::mem::transmute;
use ash::vk::make_api_version;

pub const WIN_TITLE   : &str = "penis";
pub const ENGINE_NAME : &str = "SwdwkEng";
pub const APP_NAME    : &str = "Test App";

pub const APPLICATION_VERSION: u32 = make_api_version(0, 0, 0, 0);
pub const ENGINE_VERSION     : u32 = make_api_version(0, 0, 0, 0);
pub const API_VERSION        : u32 = make_api_version(0, 0, 0, 0);
