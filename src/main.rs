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
pub mod utils;

fn main() {
    let file_path = "./atop.raw";
    atop_raw_file::parse_raw_file(file_path);
    // println!("size of sstat = {}", size_of::<SysStats>());
    // println!("size of cpustat = {}", size_of::<CpuStats>());
    // println!("size of memstat = {}", size_of::<MemStats>());
    // println!("size of netstat = {}", size_of::<NetStats>());
    // println!("size of intfstat = {}", size_of::<IntfStat>());
    // println!("size of perintf = {}", size_of::<PerIntf>());
    // println!("size of memnuma = {}", size_of::<MemNuma>());
    // println!("size of cpunuma = {}", size_of::<CpuNuma>());
    // println!("size of dskstat = {}", size_of::<DskStat>());
    // println!("size of nfsstat = {}", size_of::<NfsStat>());
    // println!("size of contstat = {}", size_of::<ContStat>());
    // println!("size of pressure = {}", size_of::<Pressure>());
    // println!("size of gpustat = {}", size_of::<GpuStat>());
    // println!("size of ifbstat = {}", size_of::<IfbStat>());
    // println!("size of llcstat = {}", size_of::<LlcStat>());
    // println!("size of wwwstat = {}", size_of::<WwwStat>());
    // println!("size of rawheader = {}", size_of::<RawHeader>());
}
