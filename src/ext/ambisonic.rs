use crate::{cstr, host::*, plugin::*};

use std::ffi::CStr;

pub const CLAP_EXT_AMBISONIC: &CStr = cstr!("clap.ambisonic/3");
pub const CLAP_EXT_AMBISONIC_COMPAT: &CStr = cstr!("clap.ambisonic.draft/3");

pub const CLAP_PORT_AMBISONIC: &CStr = cstr!("ambisonic");

pub const CLAP_AMBISONIC_ORDERING_FUMA: clap_ambisonic_ordering = 0;
pub const CLAP_AMBISONIC_ORDERING_ACN: clap_ambisonic_ordering = 1;

pub type clap_ambisonic_ordering = u32;

pub const CLAP_AMBISONIC_NORMALIZATION_MAXN: clap_ambisonic_normalization = 0;
pub const CLAP_AMBISONIC_NORMALIZATION_SN3D: clap_ambisonic_normalization = 1;
pub const CLAP_AMBISONIC_NORMALIZATION_N3D: clap_ambisonic_normalization = 2;
pub const CLAP_AMBISONIC_NORMALIZATION_SN2D: clap_ambisonic_normalization = 3;
pub const CLAP_AMBISONIC_NORMALIZATION_N2D: clap_ambisonic_normalization = 4;

pub type clap_ambisonic_normalization = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_ambisonic_config {
    pub ordering: clap_ambisonic_ordering,
    pub normalization: clap_ambisonic_normalization,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_ambisonic {
    pub is_config_supported: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            config: *const clap_ambisonic_config,
        ) -> bool,
    >,
    pub get_config: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            is_input: bool,
            port_index: u32,
            info: *mut clap_ambisonic_config,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_ambisonic {
    pub changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
}
