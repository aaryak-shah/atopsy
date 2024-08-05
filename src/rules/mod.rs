use crate::atop_raw_file::{sys_stats::SysStats, TimestampData};

pub mod cpu_rule;
pub mod engine;

pub trait InstantRule {
    fn new(threshold: f64) -> Self
    where
        Self: Sized;
    fn calculate_score(&mut self, data: &TimestampData<SysStats>) -> Result<f64, String>;
}

pub trait WindowRule {
    fn new(threshold: f64) -> Self
    where
        Self: Sized;
    fn calculate_score(&mut self, window: &[TimestampData<SysStats>]) -> Result<f64, String>;
    fn get_window_size(&self) -> usize;
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

    fn get_window_size(&self) -> usize {
        todo!()
    }

    fn calculate_score(&mut self, data: &[TimestampData<SysStats>]) -> Result<f64, String> {
        let mut total = 0.0;
        for weighted_rule in self.rules.iter_mut() {
            let rule = &mut weighted_rule.rule;
            let weight = &weighted_rule.weight;

            match rule {
                RuleType::Instant(r) => {
                    total += weight * r.calculate_score(data.last().unwrap())?
                }

                RuleType::Window(r) => total += weight * r.calculate_score(data)?,
            }
        }
        Ok(total.try_into().unwrap())
    }
}
