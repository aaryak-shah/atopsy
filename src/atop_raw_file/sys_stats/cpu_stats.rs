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
struct PerCpuStats {
    cpu_number: i32,
    sys_time: i64,
    user_time: i64,
    nice_time: i64,
    idle_time: i64,
    io_wait_time: i64,
    irq_time: i64,
    soft_irq_time: i64,
    steal_time: i64,
    guest_time: i64,
    freq_info: FreqInfo,
    instructions: i64,
    cycles: i64,
    cfuture: [i64; 6],
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
    net_cpu_stats: PerCpuStats,
    per_cpu_stats: [PerCpuStats; MAX_CPU_COUNT],
}
