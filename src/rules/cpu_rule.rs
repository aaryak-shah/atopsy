use super::{InstantRule, WindowRule};
use crate::{
    atop_raw_file::{sys_stats::SysStats, TimestampData},
    transforms::{is_above_threshold, rate_of_change},
    types::Tag,
};

pub struct CpuInstantRule {
    threshold: f64,
    tag: Tag,
}

pub struct CpuWindowRule {
    threshold: f64,
    tag: Tag,
}

const THRESHOLD: f64 = 0.1;

impl InstantRule for CpuInstantRule {
    fn new(threshold: f64, tag: Tag) -> Self {
        CpuInstantRule { threshold, tag }
    }

    fn calculate_score(&self, data: &TimestampData<SysStats>) -> Result<f64, String> {
        let mut score: f64 = 0f64;
        let net_stats = data.value.cpu_stats.net_cpu_stats;
        let idle_ratio = net_stats.idle_time as f64 / net_stats.total_cpu_time() as f64;
        let cpu_busy_ratio: f64 = 1f64 - idle_ratio;
        if is_above_threshold(cpu_busy_ratio, THRESHOLD) {
            println!("CPU%: {}", cpu_busy_ratio);
            score += (1 + (((cpu_busy_ratio - THRESHOLD) * 10f64).floor() as u64).pow(2)) as f64;
            println!("score: {}", score);
        }

        Ok(score)
    }

    fn get_tag(&self) -> Tag {
        String::from("")
    }

    fn get_threshold(&self) -> f64 {
        0.5
    }
}

impl WindowRule for CpuWindowRule {
    fn new(threshold: f64, tag: Tag) -> Self {
        CpuWindowRule { threshold, tag }
    }

    fn get_window_size(&self) -> usize {
        5
    }

    fn calculate_score(&self, window: &[TimestampData<SysStats>]) -> Result<f64, String> {
        for rate in rate_of_change(window.iter().map(|data| {
            let net_stats = data.value.cpu_stats.net_cpu_stats;
            let idle_ratio = net_stats.idle_time as f64 / net_stats.total_cpu_time() as f64;
            let busy = 1f64 - idle_ratio;
            TimestampData {
                value: busy,
                timestamp: data.timestamp,
            }
        }))? {
            if is_above_threshold(rate, self.threshold) {
                return Ok(rate - self.threshold);
            }
        }
        Ok(0f64)
    }

    fn get_tag(&self) -> Tag {
        String::from("")
    }

    fn get_threshold(&self) -> f64 {
        0.5
    }
}
