#[derive(Debug)]
#[repr(C)]
struct Server {
    netcnt: i64,
    netudpcnt: i64,
    nettcpcnt: i64,
    nettcpcon: i64,
    rpccnt: i64,
    rpcbadfmt: i64,
    rpcbadaut: i64,
    rpcbadcln: i64,
    rpcread: i64,
    rpcwrite: i64,
    rchits: i64,
    rcmiss: i64,
    rcnoca: i64,
    nrbytes: i64,
    nwbytes: i64,
    future: [i64; 8],
}

#[derive(Debug)]
#[repr(C)]
struct Client {
    rpccnt: i64,
    rpcretrans: i64,
    rpcautrefresh: i64,
    rpcread: i64,
    rpcwrite: i64,
    future: [i64; 8],
}

#[derive(Debug)]
#[repr(C)]
struct PerNfsMount {
    mountdev: [char; 128],
    age: i64,
    bytesread: i64,
    byteswrite: i64,
    bytesdread: i64,
    bytesdwrite: i64,
    bytestotread: i64,
    bytestotwrite: i64,
    pagesmread: i64,
    pagesmwrite: i64,
    future: [i64; 8],
}

#[derive(Debug)]
#[repr(C)]
struct NfsMounts {
    nrmounts: i32,
    nfsmnt: [PerNfsMount; 64],
}

#[derive(Debug)]
#[repr(C)]
pub struct NfsStat {
    server: Server,
    client: Client,
    nfsmounts: NfsMounts,
}
