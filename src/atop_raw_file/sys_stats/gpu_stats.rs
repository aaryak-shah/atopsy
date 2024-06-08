#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PerGpu {
    taskstats: i8, // using i8 to represent a bool
    nrprocs: u8,
    type_: [i8; 13],
    busid: [i8; 13],
    gpunr: i32,
    gpupercnow: i32,
    mempercnow: i32,
    memtotnow: i64,
    memusenow: i64,
    samples: i64,
    gpuperccum: i64,
    memperccum: i64,
    memusecum: i64,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GpuStat {
    nrgpus: i32,
    gpu: [PerGpu; 32],
}
