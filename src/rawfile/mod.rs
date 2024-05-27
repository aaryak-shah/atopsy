use crate::{constants::RAW_HEADER_SIZE, utils};

#[derive(Debug)]
#[repr(C)]
pub struct RawHeader {
    magic: u32,
    version: u16,
    future1: u16,
    future2: u16,
    header_length: u16,
    record_length: u16,
    clock_interrupts_persec: u16,
    pid_width: u16,
    sfuture: [u16; 5],
    sys_stats_length: u32,
    proc_stats_length: u32,
    utsname: [u8; 390],
    cfuture: [i8; 8],
    page_size: u32,
    support_flags: i32,
    os_release: i32,
    os_version: i32,
    os_subversion: i32,
    ifuture: [i32; 5],
}

impl RawHeader {
    pub fn read(buffer: [u8; RAW_HEADER_SIZE]) -> Self {
        todo!();
        // let mut buffer = [0_u8; ];
        // let bytes_read = f.read(&mut buffer).expect("Read failed");
        // assert_eq!(bytes_read, buffer.len(), "Didn't read all the bytes into memory");

        // // Safety: See below
        // let speaker: Speaker = std::mem::transmute(buffer);
        // println!("Speaker: {:?}", speaker);
        // RawHeader {
        //     magic: utils::bytes_to_u32(&buffer[0..4]),
        //     version: utils::bytes_to_u16(&buffer[4..6]),
        //     future1: 0, // 6..8
        //     future2: 0, // 8..10
        //     header_length: utils::bytes_to_u16(&buffer[10..12]),
        //     record_length: utils::bytes_to_u16(&buffer[12..14]),
        //     clock_interrupts_persec: utils::bytes_to_u16(&buffer[14..16]),
        //     pid_width: utils::bytes_to_u16(&buffer[16..18]),
        //     sfuture: [0; 5], // 18..28
        //     sys_stats_length: utils::bytes_to_u32(&buffer[28..32]),
        //     proc_stats_length: utils::bytes_to_u32(&buffer[32..36]),
        //     utsname: [0; 390], // 36..426
        //     cfuture: [0; 8],   // 426..434
        //     page_size: utils::bytes_to_u32(&buffer[434..438]),
        //     support_flags: utils::bytes_to_i32(&buffer[438..442]),
        //     os_release: utils::bytes_to_i32(&buffer[442..446]),
        //     os_version: utils::bytes_to_i32(&buffer[446..450]),
        //     os_subversion: utils::bytes_to_i32(&buffer[450..454]),
        //     ifuture: [0; 5], // 454..474
        // }
    }
}

pub struct RawRecord {
    current_time: i64,
    flags: u16,
    num_cgroups: u16,
    sfuture: [u16; 2],
    sys_stats_compressed_length: u32,
    proc_stats_compressed_length: u32,
    interval: u32,
    ndeviat: u32,  // number of tasks in list
    nactproc: u32, // number of processes in list
    task_count: u32,
    proc_count: u32,
    running_thread_count: u32,
    sleeping_thread_count: u32,
    dead_thread_count: u32,
    zombie_proc_count: u32,
    exited_proc_count: u32,
    overflow_proc_count: u32,
    idle_thread_count: u32,
    cgroup_stats_compressed_length: u32,
    cgroup_stats_length: u32,
    cgroup_pidlist_count: u32,
    pidlist_compressed_length: u32,
    ifuture: u32,
}
