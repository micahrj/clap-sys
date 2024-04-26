use crate::factory::preset_discovery::*;
use crate::{host::*, plugin::*};

use std::ffi::c_char;
use std::ffi::CStr;

pub const CLAP_EXT_PRESET_LOAD: &CStr = c"clap.preset-load/2";
pub const CLAP_EXT_PRESET_LOAD_COMPAT: &CStr = c"clap.preset-load.draft/2";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_preset_load {
    pub from_location: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            location_kind: clap_preset_discovery_location_kind,
            location: *const c_char,
            load_key: *const c_char,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_preset_load {
    pub on_error: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            location_kind: clap_preset_discovery_location_kind,
            location: *const c_char,
            load_key: *const c_char,
            os_error: i32,
            msg: *const c_char,
        ),
    >,
    pub loaded: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            location_kind: clap_preset_discovery_location_kind,
            location: *const c_char,
            load_key: *const c_char,
        ),
    >,
}
