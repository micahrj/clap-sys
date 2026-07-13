use crate::{cstr, host::clap_host, plugin::clap_plugin, stream::clap_ostream};
use core::ffi::{c_char, c_void, CStr};

pub const CLAP_EXT_WEBVIEW: &CStr = cstr!("clap.webview/3");

pub const CLAP_WINDOW_API_WEBVIEW: &CStr = cstr!("webview");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_webview {
    pub get_uri: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            uri: *mut c_char,
            uri_capacity: u32,
        ) -> i32,
    >,
    pub get_resource: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            path: *const c_char,
            mime: *mut c_char,
            mime_capacity: u32,
            data_stream: *mut clap_ostream,
        ) -> bool,
    >,
    pub receive: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, buffer: *const c_void, size: u32) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_webview {
    pub send: Option<
        unsafe extern "C" fn(host: *const clap_host, buffer: *const c_void, size: u32) -> bool,
    >,
}
