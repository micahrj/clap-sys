use crate::{cstr, plugin::clap_plugin};
use core::ffi::CStr;

pub const CLAP_EXT_BACKGROUND_ACTIVATION: &CStr = cstr!("clap.background-activation/1");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_background_activation {
    pub activate_from_background_thread: Option<
        unsafe extern "C" fn(
            plugin: *mut clap_plugin,
            sample_rate: f64,
            min_frames_count: u32,
            max_frames_count: u32,
        ) -> bool,
    >,
    pub deactivate_from_background_thread: Option<unsafe extern "C" fn(plugin: *const clap_plugin)>,
}
