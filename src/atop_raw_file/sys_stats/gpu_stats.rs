#[derive(Debug)]
#[repr(C)]
struct PerGpu {
    taskstats: i8, // using i8 to represent a bool
    nrprocs: u8,
    type_: [char; 13],
    busid: [char; 13],
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

#[derive(Debug)]
#[repr(C)]
pub struct GpuStat {
    nrgpus: i32,
    gpu: [PerGpu; 32],
}
