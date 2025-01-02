use crate::{cstr, host::*, plugin::*};

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_RESOURCE_DIRECTORY: &CStr = cstr!("clap.resource-directory/1");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_resource_directory {
    pub set_directory: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, path: *const c_char, is_shared: bool),
    >,
    pub collect: Option<unsafe extern "C" fn(plugin: *const clap_plugin, all: bool)>,
    pub get_files_count: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> u32>,
    pub get_file_path: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            index: u32,
            path: *mut c_char,
            path_size: u32,
        ) -> i32,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_resource_directory {
    pub request_directory:
        Option<unsafe extern "C" fn(host: *const clap_host, is_shared: bool) -> bool>,
    pub release_directory: Option<unsafe extern "C" fn(host: *const clap_host, is_shared: bool)>,
}
