use std::collections::HashMap;

use crate::{
    atop_raw_file::sys_stats::SysStats,
    types::{ByteOffset, Tag, UnixTimeStamp},
};

use super::{cpu_rule::CpuInstantRule, Rule, RuleGroup};

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
    stats: Vec<(UnixTimeStamp, SysStats)>,
    offsets: HashMap<UnixTimeStamp, ByteOffset>,
}

impl RuleEngine {
    pub fn new(
        rule_items: Vec<RuleEngineItem>,
        stats: Vec<(UnixTimeStamp, SysStats)>,
        offsets: HashMap<UnixTimeStamp, ByteOffset>,
    ) -> Self {
        RuleEngine {
            rule_items,
            stats,
            offsets,
        }
    }

    fn step(&mut self) -> Vec<Tag> {
        let mut tags: Vec<Tag> = vec![];
        for item in self.rule_items.iter_mut() {
            if item.rule_group.calculate_score() >= item.threshold {
                tags.push(item.tag.clone());
            }
        }
        tags
    }

    pub fn run(&mut self) {
        for (index, stat) in self.stats.iter().enumerate() {
            let cpu_usage_rule = CpuInstantRule::new(stat.1.cpu_stats.clone());
            // let cpu_rule_group = RuleGroup::new()
            todo!()
        }
    }
}
