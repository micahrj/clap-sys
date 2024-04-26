use crate::{cstr, host::*};

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_LOG: &CStr = cstr!("clap.log");

pub const CLAP_LOG_DEBUG: clap_log_severity = 0;
pub const CLAP_LOG_INFO: clap_log_severity = 1;
pub const CLAP_LOG_WARNING: clap_log_severity = 2;
pub const CLAP_LOG_ERROR: clap_log_severity = 3;
pub const CLAP_LOG_FATAL: clap_log_severity = 4;
pub const CLAP_LOG_HOST_MISBEHAVING: clap_log_severity = 5;
pub const CLAP_LOG_PLUGIN_MISBEHAVING: clap_log_severity = 6;

pub type clap_log_severity = i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_log {
    pub log: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            severity: clap_log_severity,
            msg: *const c_char,
        ),
    >,
}
