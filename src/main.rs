use std::mem::size_of;

use atop_raw_file::{
    sys_stats::{
        container_stats::ContStat,
        cpu_stats::CpuStats,
        disk_stats::DskStat,
        gpu_stats::GpuStat,
        interface_stats::{IfbStat, IntfStat, PerIntf},
        last_level_cache_stats::LlcStat,
        mem_stats::MemStats,
        net_stats::NetStats,
        nfs_stats::NfsStat,
        numa_stats::{CpuNuma, MemNuma},
        pressure::Pressure,
        www_stats::WwwStat,
        SysStats,
    },
    RawHeader,
};

pub mod atop_raw_file;
pub mod constants;
pub mod rules;
pub mod utils;

fn main() {
    let file_path = "./atop-new.raw";
    atop_raw_file::parse_raw_file(file_path);
}
