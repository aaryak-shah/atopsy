#[derive(Debug)]
#[repr(C)]
struct PerContainer {
    ctid: u32,
    numproc: u32,
    system: i64,
    user: i64,
    nice: i64,
    uptime: i64,
    physpages: i64,
}

#[derive(Debug)]
#[repr(C)]
pub struct ContStat {
    nrcontainer: i32,
    cont: [PerContainer; 128],
}
