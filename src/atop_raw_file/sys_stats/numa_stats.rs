#[derive(Debug)]
#[repr(C)]
struct MemPerNuma {
    numanr: i32,
    frag: f32,
    totmem: i64,
    freemem: i64,
    filepage: i64,
    dirtymem: i64,
    slabmem: i64,
    slabreclaim: i64,
    active: i64,
    inactive: i64,
    shmem: i64,
    tothp: i64,
    freehp: i64,
    cfuture: [i64; 2],
}

#[derive(Debug)]
#[repr(C)]
pub struct MemNuma {
    nrnuma: i64,
    numa: [MemPerNuma; 1024],
}

#[derive(Debug)]
#[repr(C)]
struct CpuPerNuma {
    numanr: i32,
    nrcpu: i64,
    stime: i64,
    utime: i64,
    ntime: i64,
    itime: i64,
    wtime: i64,
    i_time: i64,
    s_time: i64,
    steal: i64,
    guest: i64,
    cfuture: [i64; 2],
}

#[derive(Debug)]
#[repr(C)]
pub struct CpuNuma {
    nrnuma: i64,
    numa: [CpuPerNuma; 1024],
}
