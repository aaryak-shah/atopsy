use rules::engine::RuleEngine;

pub mod atop_raw_file;
pub mod constants;
pub mod rules;
pub mod transforms;
pub mod types;
pub mod utils;

fn main() {
    let file_path = "./atop-new.raw";
    let (stats, offsets) = atop_raw_file::parse_raw_file(file_path);

    let mut rule_engine: RuleEngine = RuleEngine::new(stats, offsets);

    rule_engine.run();
}
