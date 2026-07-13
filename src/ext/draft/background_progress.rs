use crate::{cstr, host::clap_host};
use core::ffi::{c_char, CStr};

pub const CLAP_EXT_BACKGROUND_PROGRESS: &CStr = cstr!("clap.background-progress/1");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_background_progress {
    pub is_canceled: Option<unsafe extern "C" fn(host: *const clap_host) -> bool>,
    pub progress:
        Option<unsafe extern "C" fn(host: *const clap_host, progress: f64, msg: *const c_char)>,
}
