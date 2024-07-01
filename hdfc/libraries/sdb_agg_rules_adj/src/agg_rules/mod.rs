use agg_rules::rule::Rule;
use agg_rules::rule::RuleBuilder;
use agg_rules::rule::RuleYieldDescriptor;
use agg_rules::rule_step::Comparator;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::Split;

mod rule;
mod rule_list;
mod rule_step;

#[derive(Debug)]
pub struct AggRules_adj {
    ord_rules: Vec<Rule>,
}

impl AggRules_adj {
    pub fn new_from_path(path: &str, reader: &Reader) -> AggRules_adj {
        return AggRulesBuilder::new_from_path(path, reader);
    }

    pub fn llg_for_acc(&self, account: &AccountWithCFs, result_count: i32) -> Vec<Option<RuleYieldDescriptor>> {
        let mut i: i32 = 0;
        let mut lst_resultant: Vec<Option<RuleYieldDescriptor>> = Vec::new();

        for rule in &self.ord_rules {
            if i < result_count {
                if let Some(rule_yield_descr) = rule.does_acc_abide(account) {
                    lst_resultant.push(Some(rule_yield_descr));
                    i += 1;
                } else {
                    continue;
                }
            }
        }
        return lst_resultant;
    }
}

#[derive(Debug)]
struct AggRulesBuilder {
    rule_builders: Vec<RuleBuilder>,
}

impl AggRulesBuilder {
    fn new_from_path(path: &str, reader: &Reader) -> AggRules_adj {
        let rdr = new_buf_rdr(path).expect("File not present");
        let mut rules = AggRulesBuilder {
            rule_builders: vec![],
        };
        for line in rdr.lines() {
            rules.build_with_intermediate_rule_step(IntermediateRuleStep::new_from_line(
                line.unwrap(),
                reader,
            ));
        }

        return rules.build();
    }

    fn build_with_intermediate_rule_step(&mut self, step: IntermediateRuleStep) {
        // Very naive, potentially n^3 complexity being achieved here.
        // But it's acceptable for the moment because:
        //  a) We want to shave off development speed
        //  b) These methods will be called once at app start-up
        //  c) Computers love vectors.
        //
        // A HashMap can be used for O(1) lookup and insertion in vec, if performance has taken a significant hit...
        for rule in self.rule_builders.iter_mut() {
            if rule.rule_id == step.rule_id {
                rule.build_with(step);
                return;
            }
        }

        let rule = RuleBuilder::new_with_first(step);
        self.rule_builders.push(rule);
    }

    fn build(self) -> AggRules_adj {
        let mut ord_rules = Vec::new();

        for rule_builder in self.rule_builders {
            ord_rules.push(rule_builder.build())
        }

        ord_rules.sort_by(|curr, next| {
            curr.sequence
                .partial_cmp(&next.sequence)
                .expect("Couldn't compare?")
        });

        return AggRules_adj { ord_rules };
    }
}

#[derive(Debug)]
pub(crate) struct IntermediateRuleStep {
    rule_id: i32,
    rule_sequence: i32,
    rule_step: i32,
    field_name: String,
    comparator: Comparator,
    connector: Connector,
}

impl IntermediateRuleStep {
    fn new_from_line(l: String, reader: &Reader) -> Self {
        let mut val_iter = l.split('|');
        let rule_id = val_iter.next().unwrap().parse().unwrap();
        let rule_ord = val_iter.next().unwrap().parse().unwrap();
        let rule_step = val_iter.next().unwrap().parse().unwrap();

        let field_name = val_iter.next().unwrap().to_string();
        let field_type = match reader.get_field_type(&field_name) {
            Some(t) => t,
            None => {
                panic!("Could not determine type of field `{}`", field_name);
            }
        };

        let comparator = Comparator::new_from_iter(&mut val_iter, field_type);
        let connector = Connector::new_from_iter(&mut val_iter);

        return IntermediateRuleStep {
            rule_id,
            rule_sequence: rule_ord,
            rule_step,
            field_name,
            comparator,
            connector,
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Connector {
    And,
    Or,
    End(i32),
}

impl Connector {
    fn new_from_iter(iter: &mut Split<char>) -> Self {
        let connector_str = iter.next().expect("Connector not present");
        match connector_str {
            "AND" => Connector::And,
            "OR" => Connector::Or,
            _ => {
                // If we can parse as an integer, it's the LLG
                let llg_opt: Result<i32, ParseIntError> = connector_str.parse();
                if let Ok(llg) = llg_opt {
                    return Connector::End(llg);
                } else {
                    panic!("Connector value not recognised");
                }
            }
        }
    }
}