use crate::{cstr, fixedpoint::*, host::*};

use core::ffi::CStr;

pub const CLAP_EXT_TRANSPORT_CONTROL: &CStr = cstr!("clap.transport-control/1");

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
    pub request_tempo: Option<unsafe extern "C" fn(host: *const clap_host, tempo: f64)>,
    pub request_time_signature:
        Option<unsafe extern "C" fn(host: *const clap_host, tsig_num: u16, tsig_denom: u16)>,
}
