#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_audio_buffer {
    pub data32: *mut *mut f32,
    pub data64: *mut *mut f64,
    pub channel_count: u32,
    pub latency: u32,
    pub constant_mask: u64,
}

unsafe impl Send for clap_audio_buffer {}
unsafe impl Sync for clap_audio_buffer {}
