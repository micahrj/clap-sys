use crate::plugin::*;

use std::ffi::{c_char, c_void, CStr};

pub const CLAP_EXT_CONFIGURABLE_AUDIO_PORTS: &CStr = c"clap.configurable-audio-ports/1";
pub const CLAP_EXT_CONFIGURABLE_AUDIO_PORTS_COMPAT: &CStr = c"clap.configurable-audio-ports.draft1";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_audio_port_configuration_request {
    pub is_input: bool,
    pub port_index: u32,
    pub channel_count: u32,
    pub port_type: *const c_char,
    pub port_details: *const c_void,
}

unsafe impl Send for clap_audio_port_configuration_request {}
unsafe impl Sync for clap_audio_port_configuration_request {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_configurable_audio_ports {
    pub can_apply_configuration: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            requests: *const clap_audio_port_configuration_request,
            request_count: u32,
        ) -> bool,
    >,
    pub apply_configuration: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            requests: *const clap_audio_port_configuration_request,
            request_count: u32,
        ) -> bool,
    >,
}
