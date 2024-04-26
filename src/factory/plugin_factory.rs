use crate::{host::*, plugin::*};

use std::ffi::c_char;
use std::ffi::CStr;

pub const CLAP_PLUGIN_FACTORY_ID: &CStr = c"clap.plugin-factory";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_factory {
    pub get_plugin_count: Option<unsafe extern "C" fn(factory: *const clap_plugin_factory) -> u32>,
    pub get_plugin_descriptor: Option<
        unsafe extern "C" fn(
            factory: *const clap_plugin_factory,
            index: u32,
        ) -> *const clap_plugin_descriptor,
    >,
    pub create_plugin: Option<
        unsafe extern "C" fn(
            factory: *const clap_plugin_factory,
            host: *const clap_host,
            plugin_id: *const c_char,
        ) -> *const clap_plugin,
    >,
}
