use agg_rules::rule_list::RSList;
use agg_rules::rule_list::RSListAbidanceRes;
use agg_rules::rule_list::RS;
use agg_rules::rule_step::RuleStep;
use agg_rules::Connector;
use agg_rules::IntermediateRuleStep;
use sdb_dyn_proto_rdr::reader::Reader;

#[derive(Debug)]
pub(crate) struct RuleBuilder {
    pub rule_id: i32,
    pub rule_sequence: i32,
    all_steps: Vec<IntermediateRuleStep>,
}

impl RuleBuilder {
    pub(crate) fn build_with(&mut self, rs: IntermediateRuleStep) {
        if self.rule_id != rs.rule_id || self.rule_sequence != rs.rule_sequence {
            panic!("Unexpected intermediate step");
        }
        self.all_steps.push(rs);
    }

    pub(crate) fn new_with_first(rs: IntermediateRuleStep) -> Self {
        return RuleBuilder {
            rule_id: rs.rule_id,
            rule_sequence: rs.rule_sequence,
            all_steps: vec![rs],
        };
    }

    pub(crate) fn build(mut self) -> Rule {
        self.all_steps.sort_by(|curr, next| {
            curr.rule_step
                .partial_cmp(&next.rule_step)
                .expect("Couldn't compare?")
        });

        // We were asked to be super defensive here.
        // So we are allowing for even the first rule step to tell us the LLG, rendering any rule steps
        // that follow, essentially useless.
        let mut llg_opt: Option<i32> = None;
        let mut rule_step_lists: Vec<RSList> = Vec::new();
        let mut rule_step_list_index = 0;

        for step in self.all_steps {
            let connector = step.connector;

            if rule_step_lists.get(rule_step_list_index).is_none() {
                rule_step_lists.push(RSList::new());
            }

            rule_step_lists[rule_step_list_index].add_rule_step(RuleBuilder::make_rule_step(step));

            match connector {
                Connector::And => {
                    // Add the next rule step in the current list; do nothing.
                    continue;
                }
                Connector::Or => {
                    // Current rule list is over, a new one should begin
                    rule_step_list_index += 1;
                    continue;
                }
                Connector::End(llg) => {
                    // We've encountered an LLG. Any rule beyond this point will be useless anyway
                    // since they will be visited later, given the steps are sorted in order.
                    llg_opt = Some(llg);
                    break;
                }
            }
        }

        if llg_opt.is_none() {
            panic!(
                "Rule number `{}` does not yield an LLG. This is not allowed.",
                self.rule_id
            );
        }

        return Rule {
            id: self.rule_id,
            sequence: self.rule_sequence,
            ord_steps: rule_step_lists,
            llg: llg_opt.unwrap(),
        };
    }

    fn make_rule_step(rs: IntermediateRuleStep) -> RS {
        RS {
            step_num: rs.rule_step,
            step: RuleStep {
                field_name: rs.field_name,
                comparator: rs.comparator,
            },
        }
    }
}

#[derive(Debug)]
pub(crate) struct Rule {
    pub(crate) id: i32,
    pub(crate) sequence: i32,
    pub(crate) ord_steps: Vec<RSList>,
    pub(crate) llg: i32,
}

impl Rule {
    pub fn does_acc_abide(&self, account: &String, reader: &Reader) -> Option<RuleYieldDescriptor> {
        for rule_list in &self.ord_steps {
            match rule_list.does_acc_abide(account, reader) {
                RSListAbidanceRes::True => {
                    let rule_yield_dscr = RuleYieldDescriptor {
                        rule_id: self.id,
                        llg: self.llg,
                    };
                    return Some(rule_yield_dscr);
                }
                RSListAbidanceRes::False => {
                    continue;
                }
            }
        }

        return None;
    }
}

pub struct RuleYieldDescriptor {
    pub rule_id: i32,
    pub llg: i32,
}
