// Instead of providing the `CLAP_VERSION_LT`, `CLAP_VERSION_EQ`, and `CLAP_VERSION_GE` macros,
// we'll derive `Eq` and `Ord` so you can make readable inline comparisons to `CLAP_VERSION`
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct clap_version {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
}

pub const CLAP_VERSION_MAJOR: u32 = 1;
pub const CLAP_VERSION_MINOR: u32 = 2;
pub const CLAP_VERSION_REVISION: u32 = 0;

pub const CLAP_VERSION: clap_version = clap_version {
    major: CLAP_VERSION_MAJOR,
    minor: CLAP_VERSION_MINOR,
    revision: CLAP_VERSION_REVISION,
};

pub const fn clap_version_is_compatible(version: clap_version) -> bool {
    version.major >= 1
}
