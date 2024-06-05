use super::{Rule, RuleGroup};

pub type Tag = String;

pub struct RuleEngineItem {
    threshold: u64,
    rule_group: RuleGroup,
    tag: Tag,
}

impl RuleEngineItem {
    pub fn new(threshold: u64, rule_group: RuleGroup, tag: Tag) -> Self {
        RuleEngineItem {
            threshold,
            rule_group,
            tag,
        }
    }
}

pub struct RuleEngine {
    rule_items: Vec<RuleEngineItem>,
}

impl RuleEngine {
    pub fn new(rule_items: Vec<RuleEngineItem>) -> Self {
        RuleEngine { rule_items }
    }

    pub fn step(&mut self) -> Vec<Tag> {
        let mut tags: Vec<Tag> = vec![];
        for item in self.rule_items.iter_mut() {
            if item.rule_group.calculate_score() >= item.threshold {
                tags.push(item.tag.clone());
            }
        }
        tags
    }
}
