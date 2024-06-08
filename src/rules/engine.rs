use std::collections::HashMap;

use crate::{
    atop_raw_file::sys_stats::SysStats,
    constants::CPU_THRESHOLD,
    types::{ByteOffset, Tag, UnixTimeStamp},
};

use super::{cpu_rule::CpuInstantRule, InstantRule, RuleGroup, RuleType, WeightedRule, WindowRule};

pub struct RuleEngineItem {
    rule_group: RuleGroup,
    tag: Tag,
}

impl RuleEngineItem {
    pub fn new(rule_group: RuleGroup, tag: Tag) -> Self {
        RuleEngineItem { rule_group, tag }
    }
}

pub struct RuleEngine {
    rule_items: Vec<RuleEngineItem>,
    stats: Vec<(UnixTimeStamp, SysStats)>,
    offsets: HashMap<UnixTimeStamp, ByteOffset>,
}

impl RuleEngine {
    pub fn new(
        stats: Vec<(UnixTimeStamp, SysStats)>,
        offsets: HashMap<UnixTimeStamp, ByteOffset>,
    ) -> Self {
        let rule_items: Vec<RuleEngineItem> = vec![RuleEngineItem::new(
            RuleGroup::with_rules(
                CPU_THRESHOLD,
                vec![WeightedRule::new(
                    1.0,
                    RuleType::Instant(Box::new(CpuInstantRule::new(CPU_THRESHOLD))),
                )],
            ),
            String::from("cpu"),
        )];

        RuleEngine {
            rule_items,
            stats,
            offsets,
        }
    }

    fn step(&mut self, data: &SysStats) -> Vec<Tag> {
        let mut tags: Vec<Tag> = vec![];
        for item in self.rule_items.iter_mut() {
            if item.rule_group.calculate_score(data) >= item.threshold {
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
