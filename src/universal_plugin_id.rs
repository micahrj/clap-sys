use std::ffi::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_universal_plugin_id {
    pub abi: *const c_char,
    pub id: *const c_char,
}

unsafe impl Send for clap_universal_plugin_id {}
unsafe impl Sync for clap_universal_plugin_id {}
