#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PerContainer {
    ctid: u64,
    numproc: u64,
    system: i64,
    user: i64,
    nice: i64,
    uptime: i64,
    physpages: i64,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ContStat {
    nrcontainer: i32,
    cont: [PerContainer; 128],
}
