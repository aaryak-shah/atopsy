use crate::atop_raw_file::sys_stats::SysStats;

pub mod cpu_rule;
pub mod engine;

pub trait InstantRule {
    fn new(threshold: f64) -> Self
    where
        Self: Sized;
    fn calculate_score(&mut self, data: &SysStats) -> f64;
}

pub trait WindowRule {
    fn new(threshold: f64) -> Self
    where
        Self: Sized;
    fn calculate_score(&mut self, data: &[SysStats]) -> f64;
}

enum RuleType {
    Instant(Box<dyn InstantRule>),
    Window(Box<dyn WindowRule>),
}

pub struct WeightedRule {
    weight: f64,
    rule: RuleType,
}

impl WeightedRule {
    pub fn new(weight: f64, rule: RuleType) -> Self {
        WeightedRule { weight, rule }
    }
}

pub struct RuleGroup {
    threshold: f64,
    rules: Vec<WeightedRule>,
}

impl RuleGroup {
    fn with_rules(threshold: f64, rules: Vec<WeightedRule>) -> Self {
        RuleGroup { threshold, rules }
    }
}

impl WindowRule for RuleGroup {
    fn new(threshold: f64) -> Self {
        RuleGroup {
            threshold,
            rules: vec![],
        }
    }

    fn calculate_score(&mut self, data: &[SysStats]) -> f64 {
        let mut total = 0.0;
        for weighted_rule in self.rules.iter_mut() {
            let rule = &mut weighted_rule.rule;
            let weight = &weighted_rule.weight;

            match rule {
                RuleType::Instant(r) => total += weight * r.calculate_score(data.last().unwrap()),
                RuleType::Window(r) => total += weight * r.calculate_score(data),
            }
        }
        total.try_into().unwrap()
    }
}
