use crate::{host::*, id::*, plugin::*};

use std::ffi::CStr;

pub const CLAP_EXT_TIMER_SUPPORT: &CStr = c"clap.timer-support";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_timer_support {
    pub on_timer: Option<unsafe extern "C" fn(plugin: *const clap_plugin, timer_id: clap_id)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_timer_support {
    pub register_timer: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            period_ms: u32,
            timer_id: *mut clap_id,
        ) -> bool,
    >,
    pub unregister_timer:
        Option<unsafe extern "C" fn(host: *const clap_host, timer_id: clap_id) -> bool>,
}
