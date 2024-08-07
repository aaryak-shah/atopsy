use std::{
    collections::{HashMap, HashSet},
    thread::current,
};

use crate::{
    atop_raw_file::{sys_stats::SysStats, TimestampData},
    constants::CPU_THRESHOLD,
    types::{ByteOffset, Tag, UnixTimeStamp},
};

use super::{
    cpu_rule::{CpuInstantRule, CpuWindowRule},
    InstantRule, RuleGroup, RuleType, WeightedRule, WindowRule,
};

pub struct RuleEngineItem {
    name: String,
    rule_group: RuleGroup,
    tags: Vec<Tag>,
}

impl RuleEngineItem {
    pub fn new(name: String, rule_group: RuleGroup) -> Self {
        RuleEngineItem {
            name,
            rule_group,
            tags: Vec::<Tag>::new(),
        }
    }
}

pub struct RuleEngine {
    rule_items: Vec<RuleEngineItem>,
    timestamped_stats: Vec<TimestampData<SysStats>>,
    offsets: HashMap<UnixTimeStamp, ByteOffset>,
    print_verbose: bool,
}

impl RuleEngine {
    pub fn new(
        stats: Vec<TimestampData<SysStats>>,
        offsets: HashMap<UnixTimeStamp, ByteOffset>,
    ) -> Self {
        let rule_items: Vec<RuleEngineItem> = vec![RuleEngineItem::new(
            String::from("CPU Rules"),
            RuleGroup::with_rules(
                CPU_THRESHOLD,
                vec![
                    WeightedRule::new(
                        1.0,
                        RuleType::Instant(Box::new(CpuInstantRule::new(
                            CPU_THRESHOLD,
                            Tag::from("CPU Instant Rule"),
                        ))),
                    ),
                    WeightedRule::new(
                        1.0,
                        RuleType::Window(Box::new(CpuWindowRule::new(
                            CPU_THRESHOLD,
                            Tag::from("CPU Window Rule"),
                        ))),
                    ),
                ],
                Tag::from("CPU Rule Group"),
            ),
        )];

        RuleEngine {
            rule_items,
            timestamped_stats: stats,
            offsets,
            print_verbose: true,
        }
    }

    fn step(&self, current_index: usize, data: &Vec<TimestampData<SysStats>>) -> Vec<Tag> {
        let mut tags: Vec<Tag> = vec![];
        for item in self.rule_items.iter() {
            match item
                .rule_group
                .calculate_score(&data.as_slice()[current_index..])
            {
                Ok(result) => {
                    if result.score >= 0.5 {
                        tags.append(&mut result.nested_tags.clone());
                    }
                }
                Err(err) => {
                    println!("Could not calculate {}", item.name);
                    if self.print_verbose {
                        println!("{}", err);
                    }
                }
            }
        }
        tags
    }

    pub fn run(&mut self) {
        for (index, time_stamp_data) in self.timestamped_stats.iter().enumerate() {
            self.step(index, &self.timestamped_stats);
        }
        todo!("MAKE SHIT RUN");
        todo!("TIMESTAMPS WHERE???");
    }
}
