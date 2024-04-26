use crate::{cstr, plugin::*};

use std::ffi::CStr;

pub const CLAP_EXT_AUDIO_PORTS_ACTIVATION: &CStr = cstr!("clap.audio-ports-activation/2");
pub const CLAP_EXT_AUDIO_PORTS_ACTIVATION_COMPAT: &CStr =
    cstr!("clap.audio-ports-activation/draft-2");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_audio_ports_activation {
    pub can_activate_while_processing:
        Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> bool>,
    pub set_active: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            is_input: bool,
            port_index: u32,
            is_active: bool,
            sample_size: u32,
        ) -> bool,
    >,
}
