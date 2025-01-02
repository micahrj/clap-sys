#![allow(non_camel_case_types)]

pub mod ext;
pub mod factory;

pub mod audio_buffer;
pub mod color;
pub mod entry;
pub mod events;
pub mod fixedpoint;
pub mod host;
pub mod id;
pub mod plugin;
pub mod plugin_features;
pub mod process;
pub mod stream;
pub mod string_sizes;
pub mod timestamp;
pub mod universal_plugin_id;
pub mod version;

/// Define a null terminated `CStr` literal.
macro_rules! cstr {
    ($str:literal) => {
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($str, "\0").as_bytes()) }
    };
}
pub(crate) use cstr;
