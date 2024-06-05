use std::time::{Duration, Instant};

use super::Rule;
use crate::atop_raw_file::sys_stats::cpu_stats::CpuStats;

pub struct CpuInstantRule {
    cpu_stats: CpuStats,
}

impl CpuInstantRule {
    pub fn new(cpu_stats: CpuStats) -> Self {
        CpuInstantRule { cpu_stats }
    }
}

const THRESHOLD: f64 = 0.1;

impl Rule for CpuInstantRule {
    fn calculate_score(&mut self) -> u64 {
        let mut score = 0;
        let net_stats = &self.cpu_stats.net_cpu_stats;
        let idle_ratio = net_stats.idle_time as f64 / net_stats.total_cpu_time() as f64;
        let cpu_busy_ratio = 1f64 - idle_ratio;
        if cpu_busy_ratio > THRESHOLD {
            println!("CPU%: {}", cpu_busy_ratio);
            score += 1 + (((cpu_busy_ratio - THRESHOLD) * 10f64).floor() as u64).pow(2);
            println!("score: {}", score);
        }

        score
    }
}
