#[derive(Debug)]
#[repr(C)]
struct PerIntf {
    name: [char; 16],
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
    type_: char,
    speed: i32,
    speedp: i32,
    duplex: char,
    cfuture: [i64; 4],
}

#[derive(Debug)]
#[repr(C)]
pub struct IntfStat {
    nrintf: i32,
    intf: [PerIntf; 128],
}

#[derive(Debug)]
#[repr(C)]
struct PerIfb {
    ibname: [char; 12],
    portnr: i16,
    lanes: i16,
    rate: i64,
    rcvb: i64,
    sndb: i64,
    rcvp: i64,
    sndp: i64,
    cfuture: [i64; 4],
}

#[derive(Debug)]
#[repr(C)]
pub struct IfbStat {
    nrports: i32,
    ifb: [PerIfb; 32],
}
