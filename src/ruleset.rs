use {
    crate::{
        rule::Rule,
    },
};

/// all the rules of a watcher, that is the rules
/// related to an input queue
#[derive(Debug)]
pub struct Ruleset {
    pub rules: Vec<Rule>,
}

impl Ruleset {
    pub fn matching_rules(&self, task: &str) -> Vec<&Rule> {
        self.rules.iter().filter(|r| r.is_match(&task)).collect()
    }
}
