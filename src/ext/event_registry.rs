use crate::{cstr, host::*};

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_EVENT_REGISTRY: &CStr = cstr!("clap.event-registry");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_event_registry {
    pub query: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            space_name: *const c_char,
            space_id: *mut u16,
        ) -> bool,
    >,
}
