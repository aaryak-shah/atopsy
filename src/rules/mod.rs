use crate::{
    atop_raw_file::{sys_stats::SysStats, TimestampData},
    types::Tag,
};

pub mod cpu_rule;
pub mod engine;

pub trait InstantRule {
    fn new(threshold: f64, tag: Tag) -> Self
    where
        Self: Sized;
    fn calculate_score(&self, data: &TimestampData<SysStats>) -> Result<f64, String>;
    fn get_tag(&self) -> Tag;
    fn get_threshold(&self) -> f64;
}

pub trait WindowRule {
    fn new(threshold: f64, tag: Tag) -> Self
    where
        Self: Sized;
    fn calculate_score(&self, window: &[TimestampData<SysStats>]) -> Result<f64, String>;
    fn get_window_size(&self) -> usize;
    fn get_tag(&self) -> Tag;
    fn get_threshold(&self) -> f64;
}

pub enum RuleType {
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
    tag: Tag,
    nested_tags: Vec<Tag>,
}

pub struct GroupResult {
    nested_tags: Vec<Tag>,
    score: f64,
}

impl RuleGroup {
    fn with_rules(threshold: f64, rules: Vec<WeightedRule>, tag: Tag) -> Self {
        RuleGroup {
            threshold,
            rules,
            tag,
            nested_tags: Vec::new(),
        }
    }

    pub fn get_nested_tags(&self) -> Vec<Tag> {
        self.nested_tags.clone()
    }
}

impl RuleGroup {
    fn new(threshold: f64, tag: Tag) -> Self {
        RuleGroup {
            threshold,
            rules: vec![],
            tag,
            nested_tags: Vec::new(),
        }
    }

    fn get_window_size(&self) -> usize {
        todo!()
    }

    fn calculate_score(&self, data: &[TimestampData<SysStats>]) -> Result<GroupResult, String> {
        let mut total = 0.0;
        let mut nested_tags = Vec::<Tag>::new();
        for weighted_rule in self.rules.iter() {
            let rule = &weighted_rule.rule;
            let weight = &weighted_rule.weight;
            let (score, tag) = match rule {
                RuleType::Instant(r) => (r.calculate_score(data.last().unwrap())?, r.get_tag()),
                RuleType::Window(r) => {
                    let window_size = r.get_window_size();
                    let window = &data[..window_size];
                    (r.calculate_score(&window)?, r.get_tag())
                }
            };
            if score >= self.threshold {
                nested_tags.push(tag);
            }
            total += weight * score;
        }
        Ok(GroupResult {
            nested_tags,
            score: total.try_into().unwrap(),
        })
    }
}
