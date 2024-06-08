#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct PerIntf {
    pub name: [i8; 16],
    rbyte: i64,
    rpack: i64,
    rerrs: i64,
    rdrop: i64,
    rfifo: i64,
    rframe: i64,
    rcompr: i64,
    rmultic: i64,
    rfuture: [i64; 4],
    sbyte: i64,
    spack: i64,
    serrs: i64,
    sdrop: i64,
    sfifo: i64,
    scollis: i64,
    scarrier: i64,
    scompr: i64,
    sfuture: [i64; 4],
    type_: i8,
    speed: i64,
    speedp: i64,
    duplex: i8,
    cfuture: [i64; 4],
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IntfStat {
    nrintf: i32,
    pub intf: [PerIntf; 128],
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PerIfb {
    ibname: [i8; 12],
    portnr: i16,
    lanes: i16,
    rate: i64,
    rcvb: i64,
    sndb: i64,
    rcvp: i64,
    sndp: i64,
    cfuture: [i64; 4],
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IfbStat {
    nrports: i32,
    ifb: [PerIfb; 32],
}
