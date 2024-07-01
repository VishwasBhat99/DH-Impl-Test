use agg_rules::rule_step::RuleStep;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

#[derive(Debug)]
/// One set of `AND` joined RuleSteps in a Rule
pub(crate) struct RSList {
    steps: Vec<RS>,
}

impl RSList {

    pub(crate) fn new() -> Self {
        RSList {
            steps: Vec::new()
        }
    }

    pub(crate) fn add_rule_step(&mut self, rs: RS) {
        self.steps.push(rs)
    }

    pub(crate) fn does_acc_abide(&self, account: &AccountWithCFs) -> RSListAbidanceRes {

        for step in &self.steps {
            if step.does_acc_abide(account) == true {
                continue;
            } else {
                return RSListAbidanceRes::False;
            }
        }

        return RSListAbidanceRes::True;
    }
}

pub enum RSListAbidanceRes {
    False,
    True,
}

#[derive(Debug)]
pub(crate) struct RS {
    pub(crate) step_num: i32,
    pub(crate) step: RuleStep,
}

impl RS {
    pub fn does_acc_abide(&self, account: &AccountWithCFs) -> bool {
        self.step.does_acc_abide(account)
    }
}
