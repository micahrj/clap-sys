use crate::{cstr, host::clap_host, id::clap_id};
use core::ffi::CStr;

pub const CLAP_EXT_PARAM_HOVERED: &CStr = cstr!("clap.param-hovered/1");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_param_hovered {
    pub update: Option<unsafe extern "C" fn(host: *const clap_host, hovered_param_id: clap_id)>,
}
