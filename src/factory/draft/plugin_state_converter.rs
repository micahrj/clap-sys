use std::ffi::{c_char, c_void, CStr};

use crate::factory::preset_discovery::clap_universal_plugin_id;
use crate::{
    cstr,
    id::clap_id,
    stream::{clap_istream, clap_ostream},
    version::clap_version,
};

pub const CLAP_PLUGIN_STATE_CONVERTER_FACTORY_ID: &CStr =
    cstr!("clap.plugin-state-converter-factory/1");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_state_converter_descriptor {
    pub clap_version: clap_version,
    pub src_plugin_id: clap_universal_plugin_id,
    pub dst_plugin_id: clap_universal_plugin_id,
    pub id: *const c_char,
    pub name: *const c_char,
    pub vendor: *const c_char,
    pub version: *const c_char,
    pub description: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_state_converter {
    pub desc: *const clap_plugin_state_converter_descriptor,
    pub converter_data: *mut c_void,
    pub destroy: Option<unsafe extern "C" fn(converter: *mut clap_plugin_state_converter)>,
    pub convert_state: Option<
        unsafe extern "C" fn(
            converter: *mut clap_plugin_state_converter,
            src: *const clap_istream,
            dst: *const clap_ostream,
            error_buffer: *mut c_char,
            error_buffer_size: usize,
        ) -> bool,
    >,
    pub convert_normalized_value: Option<
        unsafe extern "C" fn(
            converter: *mut clap_plugin_state_converter,
            src_param_id: clap_id,
            src_normalized_value: f64,
            dst_param_id: *mut clap_id,
            dst_normalized_value: *mut f64,
        ) -> bool,
    >,
    pub convert_plain_value: Option<
        unsafe extern "C" fn(
            converter: *mut clap_plugin_state_converter,
            src_param_id: clap_id,
            src_plain_value: f64,
            dst_param_id: *mut clap_id,
            dst_plain_value: *mut f64,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_state_converter_factory {
    pub count:
        Option<unsafe extern "C" fn(factory: *mut clap_plugin_state_converter_factory) -> u32>,
    pub get_descriptor: Option<
        unsafe extern "C" fn(
            factory: *const clap_plugin_state_converter_factory,
            index: u32,
        ) -> *const clap_plugin_state_converter_descriptor,
    >,
    pub create: Option<
        unsafe extern "C" fn(
            factory: *const clap_plugin_state_converter_factory,
            converter_id: *const c_char,
        ) -> *mut clap_plugin_state_converter,
    >,
}
