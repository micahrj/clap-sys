use crate::{
    cstr,
    plugin::clap_plugin,
    stream::{clap_istream, clap_ostream},
};
use core::ffi::CStr;

pub const CLAP_EXT_BACKGROUND_STATE_CONTEXT: &CStr = cstr!("clap.background-state-context/1");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_background_state_context {
    pub save_from_background_thread: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            stream: *const clap_ostream,
            context_type: u32,
        ) -> bool,
    >,
    pub load_from_background_thread: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            stream: *const clap_istream,
            context_type: u32,
        ) -> bool,
    >,
}
