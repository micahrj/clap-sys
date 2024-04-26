use crate::{host::*, plugin::*};

use std::ffi::CStr;

pub const CLAP_EXT_VOICE_INFO: &CStr = c"clap.voice-info";

pub const CLAP_VOICE_INFO_SUPPORTS_OVERLAPPING_NOTES: u64 = 1 << 0;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_voice_info {
    pub voice_count: u32,
    pub voice_capacity: u32,
    pub flags: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_voice_info {
    pub get: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, info: *mut clap_voice_info) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_voice_info {
    pub changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
}
