pub mod sys_stats;
pub mod utsname;

use std::{
    fs::File,
    io::{BufReader, Read},
    mem::size_of,
};

use sys_stats::SysStats;
use utsname::UTSName;

use crate::{
    constants::{MAGIC, RAW_HEADER_SIZE, RAW_RECORD_SIZE},
    rules::{
        cpu_rule::{self, CpuInstantRule},
        engine::{RuleEngine, RuleEngineItem, Tag},
        RuleGroup,
    },
};

#[derive(Debug)]
#[repr(C)]
pub struct RawHeader {
    pub magic: u32,
    pub version: u16,
    pub future1: u16,
    pub future2: u16,
    pub header_length: u16,
    pub record_length: u16,
    pub clock_interrupts_persec: u16,
    pub pid_width: u16,
    pub sfuture: [u16; 5],
    pub sys_stats_length: u32,
    pub proc_stats_length: u32,
    pub utsname: UTSName,
    pub cfuture: [i8; 8],
    pub page_size: u32,
    pub support_flags: i32,
    pub os_release: i32,
    pub os_version: i32,
    pub os_subversion: i32,
    pub cgroups_stats_length: u32,
    pub ifuture: [i32; 5],
}

impl RawHeader {
    pub unsafe fn from(buffer: [u8; RAW_HEADER_SIZE]) -> Self {
        std::mem::transmute(buffer)
    }
}

#[derive(Debug)]
#[repr(C)]
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

impl RawRecord {
    pub unsafe fn from(buffer: [u8; RAW_RECORD_SIZE]) -> Self {
        std::mem::transmute(buffer)
    }
}

fn read_raw_header(reader: &mut BufReader<File>) -> RawHeader {
    let mut raw_header_buffer = [0u8; RAW_HEADER_SIZE];
    reader.read_exact(&mut raw_header_buffer).unwrap();
    unsafe { RawHeader::from(raw_header_buffer) }
}

fn read_raw_record(reader: &mut BufReader<File>) -> RawRecord {
    let mut raw_record_buffer = [0u8; RAW_RECORD_SIZE];
    reader.read_exact(&mut raw_record_buffer).unwrap();
    unsafe { RawRecord::from(raw_record_buffer) }
}

fn read_sys_stats(reader: &mut BufReader<File>, compressed_length: u32) -> SysStats {
    let mut compressed_stats_buffer = Vec::<u8>::new();
    compressed_stats_buffer.resize(compressed_length as usize, 0u8);
    reader
        .read_exact(compressed_stats_buffer.as_mut_slice())
        .unwrap();
    unsafe { SysStats::from(compressed_stats_buffer) }
}

pub fn parse_raw_file(file_path: &str) {
    // load file descriptor
    let raw_file = File::open(file_path).unwrap();

    // use buffered reader to parse file in chunks
    let mut buf_reader = BufReader::new(raw_file);

    let raw_header = read_raw_header(&mut buf_reader);
    assert_eq!(raw_header.magic, MAGIC, "File is corrupted");

    loop {
        let raw_record = read_raw_record(&mut buf_reader);
        let sys_stats = read_sys_stats(&mut buf_reader, raw_record.sys_stats_compressed_length);

        if (raw_header.sys_stats_length != size_of::<SysStats>() as u32) {
            panic!("mismatching length {}", raw_header.sys_stats_length)
        }

        let cpu_rule = CpuInstantRule::new(sys_stats.cpu_stats);
        let rule_group = RuleGroup::new(vec![(1, Box::new(cpu_rule))]);
        let rule_engine_item = RuleEngineItem::new(1, rule_group, Tag::from("CPU bad"));
        let mut rule_engine = RuleEngine::new(vec![rule_engine_item]);
        let tags = rule_engine.step();
        println!("tags: {:?}", tags);
        // println!(
        //     "{:#?}",
        //     (sys_stats.mem_stats.physmem) * (raw_header.page_size as i64 / 1024) / 1024
        // );
        break;
    }
}
