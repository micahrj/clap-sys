use crate::{cstr, plugin::*, stream::*};

use std::ffi::CStr;

pub const CLAP_EXT_STATE_CONTEXT: &CStr = cstr!("clap.state-context/2");

pub const CLAP_STATE_CONTEXT_FOR_PRESET: clap_plugin_state_context_type = 1;
pub const CLAP_STATE_CONTEXT_FOR_DUPLICATE: clap_plugin_state_context_type = 2;
pub const CLAP_STATE_CONTEXT_FOR_PROJECT: clap_plugin_state_context_type = 3;

pub type clap_plugin_state_context_type = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_state_context {
    pub save: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            stream: *const clap_ostream,
            context_type: clap_plugin_state_context_type,
        ) -> bool,
    >,
    pub load: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            stream: *const clap_istream,
            context_type: clap_plugin_state_context_type,
        ) -> bool,
    >,
}
