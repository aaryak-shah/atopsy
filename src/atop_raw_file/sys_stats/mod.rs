pub mod cpu_stats;
pub mod mem_stats;

use std::io::Read;

use crate::constants::SYS_STATS_SIZE;
use cpu_stats::CpuStats;
use flate2::read::ZlibDecoder;
use mem_stats::MemStats;

#[derive(Debug)]
#[repr(C)]
pub struct SysStats {
    cpu_stats: CpuStats,
    mem_stats: MemStats,
}

impl SysStats {
    pub unsafe fn from(compressed: Vec<u8>) -> Self {
        let mut sys_stats_buffer = [0u8; SYS_STATS_SIZE];
        let mut decompresser = ZlibDecoder::new(compressed.as_slice());
        decompresser.read_exact(&mut sys_stats_buffer).unwrap();
        std::mem::transmute(sys_stats_buffer)
    }
}
