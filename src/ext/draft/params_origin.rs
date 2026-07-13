use crate::{cstr, host::clap_host, id::clap_id, plugin::clap_plugin};
use core::ffi::CStr;

pub const CLAP_EXT_PARAMS_ORIGIN: &CStr = cstr!("clap.params-origin/1");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_params_origin {
    pub get: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            param_id: clap_id,
            out_value: *mut f64,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_params_origin {
    pub changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
}
