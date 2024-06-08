pub mod container_stats;
pub mod cpu_stats;
pub mod disk_stats;
pub mod gpu_stats;
pub mod interface_stats;
pub mod last_level_cache_stats;
pub mod mem_stats;
pub mod net_stats;
pub mod nfs_stats;
pub mod numa_stats;
pub mod pressure;
pub mod www_stats;

use std::{io::Read, mem::size_of};

use crate::constants::SYS_STATS_SIZE;
use container_stats::ContStat;
use cpu_stats::CpuStats;
use disk_stats::DskStat;
use flate2::{read::ZlibDecoder, Decompress};
use gpu_stats::GpuStat;
use interface_stats::{IfbStat, IntfStat};
use last_level_cache_stats::LlcStat;
use mem_stats::MemStats;
use net_stats::NetStats;
use nfs_stats::NfsStat;
use numa_stats::{CpuNuma, MemNuma};
use pressure::Pressure;
use www_stats::WwwStat;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct SysStats {
    pub cpu_stats: CpuStats,
    pub mem_stats: MemStats,
    net_stats: NetStats,
    pub interface_stats: IntfStat,
    memnuma: MemNuma,
    cpunuma: CpuNuma,
    pub dsk: DskStat,
    nfs: NfsStat,
    cfs: ContStat,
    psi: Pressure,
    gpu: GpuStat,
    ifb: IfbStat,
    llc: LlcStat,
    www: WwwStat,
}

impl SysStats {
    pub unsafe fn from(compressed: Vec<u8>) -> Self {
        println!("compressed length: {}", compressed.len());
        let mut sys_stats_buffer = [0u8; size_of::<SysStats>()];
        let mut decompresser = Decompress::new(true);
        decompresser
            .decompress(
                &compressed.as_slice(),
                &mut sys_stats_buffer,
                flate2::FlushDecompress::Sync,
            )
            .unwrap();
        std::mem::transmute(sys_stats_buffer)
    }
}
