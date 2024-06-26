#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PerLlc {
    id: u8,
    occupancy: f32,
    mbm_local: i64,
    mbm_total: i64,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct LlcStat {
    nrllcs: i32,
    perllc: [PerLlc; 256],
}
