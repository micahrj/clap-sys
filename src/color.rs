pub const CLAP_COLOR_TRANSPARENT: clap_color = clap_color {
    alpha: 0,
    red: 0,
    green: 0,
    blue: 0,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_color {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
