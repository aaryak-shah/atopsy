use super::InstantRule;
use crate::atop_raw_file::sys_stats::SysStats;

pub struct CpuInstantRule {
    threshold: f64,
}

const THRESHOLD: f64 = 0.1;

impl InstantRule for CpuInstantRule {
    fn new(threshold: f64) -> Self {
        CpuInstantRule { threshold }
    }

    fn calculate_score(&mut self, data: &SysStats) -> u64 {
        let mut score = 0;
        let net_stats = data.cpu_stats.net_cpu_stats;
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
