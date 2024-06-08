use crate::atop_raw_file::sys_stats::{self, SysStats};

pub mod cpu_rule;
pub mod engine;

pub trait Rule {
    fn calculate_score(&mut self) -> u64;
    // fn set_data(&mut self, sys_stats: &SysStats);
}

pub struct RuleGroup {
    rules: Vec<(u64, Box<dyn Rule>)>,
}

impl RuleGroup {
    pub fn new(rules: Vec<(u64, Box<dyn Rule>)>) -> Self {
        RuleGroup { rules }
    }
}

impl Rule for RuleGroup {
    fn calculate_score(&mut self) -> u64 {
        let mut total = 0;
        for (weight, rule) in self.rules.iter_mut() {
            total += *weight * rule.calculate_score();
        }
        total
    }
}
