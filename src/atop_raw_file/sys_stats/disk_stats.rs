#[derive(Debug)]
#[repr(C)]
struct PerDsk {
    name: [i8; 32],
    nread: i64,
    nrsect: i64,
    nwrite: i64,
    nwsect: i64,
    io_ms: i64,
    avque: i64,
    ndisc: i64,
    ndsect: i64,
    inflight: i64,
    cfuture: [i64; 3],
}

#[derive(Debug)]
#[repr(C)]
pub struct DskStat {
    ndsk: i32,
    nmdd: i32,
    nlvm: i32,
    dsk: [PerDsk; 1024],
    mdd: [PerDsk; 256],
    lvm: [PerDsk; 2048],
}
