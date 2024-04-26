use crate::{cstr, plugin::*};

use std::ffi::CStr;

pub const CLAP_EXT_RENDER: &CStr = cstr!("clap.render");

pub const CLAP_RENDER_REALTIME: clap_plugin_render_mode = 0;
pub const CLAP_RENDER_OFFLINE: clap_plugin_render_mode = 1;

pub type clap_plugin_render_mode = i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_render {
    pub has_hard_realtime_requirement:
        Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> bool>,
    pub set: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, mode: clap_plugin_render_mode) -> bool,
    >,
}
