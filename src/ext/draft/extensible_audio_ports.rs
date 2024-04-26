use crate::plugin::*;

use std::ffi::{c_char, c_void, CStr};

pub const CLAP_EXT_EXTENSIBLE_AUDIO_PORTS: &CStr = c"clap.extensible-audio-ports.draft0";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_extensible_audio_ports {
    pub add_port: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            is_input: bool,
            channel_count: u32,
            port_type: *const c_char,
            port_details: *const c_void,
        ) -> bool,
    >,
    pub remove_port: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, is_input: bool, index: u32) -> bool,
    >,
}
