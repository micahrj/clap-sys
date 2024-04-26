use crate::{color::*, cstr, host::*, plugin::*, string_sizes::*};

use std::ffi::c_char;
use std::ffi::CStr;

pub const CLAP_EXT_TRACK_INFO: &CStr = cstr!("clap.track-info/1");
pub const CLAP_EXT_TRACK_INFO_COMPAT: &CStr = cstr!("clap.track-info.draft/1");

pub const CLAP_TRACK_INFO_HAS_TRACK_NAME: u64 = 1 << 0;
pub const CLAP_TRACK_INFO_HAS_TRACK_COLOR: u64 = 1 << 1;
pub const CLAP_TRACK_INFO_HAS_AUDIO_CHANNEL: u64 = 1 << 2;
pub const CLAP_TRACK_INFO_IS_FOR_RETURN_TRACK: u64 = 1 << 3;
pub const CLAP_TRACK_INFO_IS_FOR_BUS: u64 = 1 << 4;
pub const CLAP_TRACK_INFO_IS_FOR_MASTER: u64 = 1 << 5;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_track_info {
    pub flags: u64,
    pub name: [c_char; CLAP_NAME_SIZE],
    pub color: clap_color,
    pub audio_channel_count: i32,
    pub audio_port_type: *const c_char,
}

unsafe impl Send for clap_track_info {}
unsafe impl Sync for clap_track_info {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_track_info {
    pub changed: Option<unsafe extern "C" fn(plugin: *const clap_plugin)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_track_info {
    pub get:
        Option<unsafe extern "C" fn(host: *const clap_host, info: *mut clap_track_info) -> bool>,
}
