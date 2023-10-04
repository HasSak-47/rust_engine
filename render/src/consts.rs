
use std::ffi::CStr;
use std::mem::transmute;

pub const C_WIN_TITLE   : &'static CStr = unsafe {transmute("penis\0")};
pub const C_ENGINE_NAME : &'static CStr = unsafe {transmute("SwdwkEng\0")};
pub const C_APP_NAME    : &'static CStr = unsafe {transmute("Test App\0")};

pub const WIN_TITLE   : &str = "penis";
pub const ENGINE_NAME : &str = "SwdwkEng";
pub const APP_NAME    : &str = "Test App";
