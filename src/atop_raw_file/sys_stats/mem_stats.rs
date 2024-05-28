#[derive(Debug)]
#[repr(C)]
pub struct MemStats {
    physmem: i64,       // number of physical pages
    freemem: i64,       // number of free     pages
    buffermem: i64,     // number of buffer   pages
    slabmem: i64,       // number of slab     pages
    cachemem: i64,      // number of cache    pages
    cachedrt: i64,      // number of cache    pages (dirty)
    totswap: i64,       // number of pages in swap
    freeswap: i64,      // number of free swap pages
    pgscans: i64,       // number of page scans
    pgsteal: i64,       // number of page steals
    allocstall: i64,    // try to free pages forced
    swouts: i64,        // number of pages swapped out
    swins: i64,         // number of pages swapped in
    tcpsock: i64,       // number of pages allocated by TCP sockets
    udpsock: i64,       // number of pages allocated by UDP sockets
    commitlim: i64,     // commit limit in pages
    committed: i64,     // number of reserved pages
    shmem: i64,         // tot shmem incl. tmpfs (pages)
    shmrss: i64,        // resident shared memory (pages)
    shmswp: i64,        // swapped shared memory (pages)
    slabreclaim: i64,   // reclaimable slab (pages)
    stothugepage: i64,  // total huge pages (huge pages) - small
    sfreehugepage: i64, // free  huge pages (huge pages) - small
    shugepagesz: i64,   // huge page size (bytes) - small
    vmwballoon: i64,    // vmware claimed balloon pages
    zfsarcsize: i64,    // zfsonlinux ARC size (pages)
    swapcached: i64,    // swap cache (pages)
    ksmsharing: i64,    // saved i.e. deduped memory (pages)
    ksmshared: i64,     // current size shared pages (pages)
    zswapped: i64,      // zswap stored pages decompressed (pages)
    zswap: i64,         // zswap current pool size compressed (pages)
    oomkills: i64,      // number of oom killings
    compactstall: i64,  // counter for process stalls
    pgmigrate: i64,     // counter for migrated successfully (pages)
    numamigrate: i64,   // counter for numa migrated (pages)
    pgouts: i64,        // total number of pages written to block device
    pgins: i64,         // total number of pages read from block device
    pagetables: i64,    // page tables of processes (pages)
    zswouts: i64,       // number of pages swapped out to zswap
    zswins: i64,        // number of pages swapped in from zswap
    ltothugepage: i64,  // total huge pages (huge pages) - large
    lfreehugepage: i64, // free  huge pages (huge pages) - large
    lhugepagesz: i64,   // huge page size (bytes) - large
    availablemem: i64,  // available memory (pages)
    anonhugepage: i64,  // anonymous transparent huge pages // (in units of 'normal' pages)
    cfuture: [i64; 5],  // reserved for future use
}
