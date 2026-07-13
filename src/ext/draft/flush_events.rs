use crate::{
    cstr, events::clap_input_events, events::clap_output_events, host::clap_host,
    plugin::clap_plugin,
};
use core::ffi::CStr;

pub const CLAP_EXT_FLUSH_EVENTS: &CStr = cstr!("clap.flush-events/1");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_flush_events {
    pub flush: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            in_events: *const clap_input_events,
            out_events: *const clap_output_events,
        ),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_flush_events {
    pub request_flush: Option<unsafe extern "C" fn(host: *const clap_host)>,
}
