#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct WwwStat {
    accesses: i64,
    totkbytes: i64,
    uptime: i64,
    bworkers: i32,
    iworkers: i32,
}
