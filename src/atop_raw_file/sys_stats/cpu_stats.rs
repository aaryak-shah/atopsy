use crate::constants::MAX_CPU_COUNT;

#[derive(Debug)]
#[repr(C)]
struct FreqInfo {
    max_frequency_mhz: i64,
    cnt: i64,
    ticks: i64,
}

#[derive(Debug)]
#[repr(C)]
pub struct PerCpuStats {
    cpu_number: i32,
    pub sys_time: i64,
    pub user_time: i64,
    pub nice_time: i64,
    pub idle_time: i64,
    pub io_wait_time: i64,
    pub irq_time: i64,
    pub soft_irq_time: i64,
    pub steal_time: i64,
    pub guest_time: i64,
    freq_info: FreqInfo,
    instructions: i64,
    cycles: i64,
    cfuture: [i64; 6],
}

impl PerCpuStats {
    pub fn total_cpu_time(&self) -> i64 {
        self.sys_time
            + self.user_time
            + self.idle_time
            + self.nice_time
            + self.io_wait_time
            + self.irq_time
            + self.soft_irq_time
            + self.steal_time
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CpuStats {
    pub cpu_count: i64,
    device_interrupts_count: i64,
    context_switch_count: i64,
    started_procs_count: i64,
    load_avg_1m: f32,
    load_avg_5m: f32,
    load_avg_15m: f32,
    cfuture: [i64; 4],
    pub net_cpu_stats: PerCpuStats,
    per_cpu_stats: [PerCpuStats; MAX_CPU_COUNT],
}
