use crate::{host::*, plugin::*};

use std::ffi::CStr;

pub const CLAP_EXT_LATENCY: &CStr = c"clap.latency";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_latency {
    pub get: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> u32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_latency {
    pub changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
}
