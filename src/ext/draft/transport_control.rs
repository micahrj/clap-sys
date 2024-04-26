use crate::{fixedpoint::*, host::*};

use std::ffi::CStr;

pub const CLAP_EXT_TRANSPORT_CONTROL: &CStr = c"clap.transport-control.draft/0";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_transport_control {
    pub request_start: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub request_stop: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub request_continue: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub request_pause: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub request_toggle_play: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub request_jump: Option<unsafe extern "C" fn(host: *const clap_host, position: clap_beattime)>,
    pub request_loop_region: Option<
        unsafe extern "C" fn(host: *const clap_host, start: clap_beattime, duration: clap_beattime),
    >,
    pub request_toggle_loop: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub request_enable_loop: Option<unsafe extern "C" fn(host: *const clap_host, is_enabled: bool)>,
    pub request_record: Option<unsafe extern "C" fn(host: *const clap_host, is_recording: bool)>,
    pub request_toggle_record: Option<unsafe extern "C" fn(host: *const clap_host)>,
}
