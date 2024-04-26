use crate::{cstr, host::*, plugin::*, stream::*};

use std::ffi::CStr;

pub const CLAP_EXT_STATE: &CStr = cstr!("clap.state");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_state {
    pub save: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, stream: *const clap_ostream) -> bool,
    >,
    pub load: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, stream: *const clap_istream) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_state {
    pub mark_dirty: Option<unsafe extern "C" fn(host: *const clap_host)>,
}
