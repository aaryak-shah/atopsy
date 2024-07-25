use super::{InstantRule, WindowRule};
use crate::{
    atop_raw_file::sys_stats::SysStats, constants::CPU_THRESHOLD, transforms::is_above_threshold,
};

pub struct CpuInstantRule {
    threshold: f64,
}

pub struct CpuWindowRule {
    threshold: f64,
}

const THRESHOLD: f64 = 0.1;

impl InstantRule for CpuInstantRule {
    fn new(threshold: f64) -> Self {
        CpuInstantRule { threshold }
    }

    fn calculate_score(&mut self, data: &SysStats) -> f64 {
        let mut score: f64 = 0f64;
        let net_stats = data.cpu_stats.net_cpu_stats;
        let idle_ratio = net_stats.idle_time as f64 / net_stats.total_cpu_time() as f64;
        let cpu_busy_ratio: f64 = 1f64 - idle_ratio;
        if is_above_threshold(cpu_busy_ratio, THRESHOLD) {
            println!("CPU%: {}", cpu_busy_ratio);
            score += (1 + (((cpu_busy_ratio - THRESHOLD) * 10f64).floor() as u64).pow(2)) as f64;
            println!("score: {}", score);
        }

        score
    }
}

impl WindowRule for CpuWindowRule {
    fn new(threshold: f64) -> Self {
        CpuWindowRule { threshold }
    }

    fn get_window_size(&self) -> usize {
        5
    }

    fn calculate_score(&mut self, window: &[SysStats]) -> f64 {

        window.iter().map(|data| {
            let net_stats = data.cpu_stats.net_cpu_stats;
            let idle_ratio = net_stats.idle_time as f64 / net_stats.total_cpu_time() as f64;
            1f64 - idle_ratio
        })
    }
}
