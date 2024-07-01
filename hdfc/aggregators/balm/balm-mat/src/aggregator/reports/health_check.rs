use aggregator::llg_key::LLGKey;
use aggregator::reports::llgs_report::PrincipalInterestAmountReport;
use aggregator::reports::AggregationReport;
use float_cmp::ApproxEqUlps;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;

pub(super) struct HealthCheckReport {
    input_to_llgs_account_count_difference: i32,
    input_to_llgs_cashflow_count_difference: i32,
    input_to_llgs_principal_amount_difference: f64,
    input_to_llgs_interest_amount_difference: f64,
    llgs_with_mismatched_principal_amount_file_distribution: Vec<LLGKey>,
    llgs_with_mismatched_interest_amount_file_distribution: Vec<LLGKey>,
    llgs_with_summary_mismatches: HashMap<LLGKey, PrincipalInterestAmountReport>,
    groups_with_incorrect_output_records: Vec<String>,
}

impl Serialize for HealthCheckReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("HealthCheckReport", 8)?;
        s.serialize_field(
            "inputToLLGsAccountCountDifference",
            &self.input_to_llgs_account_count_difference,
        )?;
        s.serialize_field(
            "inputToLLGsCashflowCountDifference",
            &self.input_to_llgs_cashflow_count_difference,
        )?;
        s.serialize_field(
            "inputToLLGsPrincipalAmountDifference",
            &self.input_to_llgs_principal_amount_difference,
        )?;
        s.serialize_field(
            "inputToLLGsInterestAmountDifference",
            &self.input_to_llgs_interest_amount_difference,
        )?;
        s.serialize_field(
            "llgsWithMismatchedPrincipalAmountFileDistribution",
            &llgs_to_string_vec(&self.llgs_with_mismatched_principal_amount_file_distribution),
        )?;
        s.serialize_field(
            "llgsWithMismatchedInterestAmountFileDistribution",
            &llgs_to_string_vec(&self.llgs_with_mismatched_interest_amount_file_distribution),
        )?;
        s.serialize_field(
            "llgsWithSummaryMismatches",
            &llg_map_to_string_map(&self.llgs_with_summary_mismatches),
        )?;
        s.serialize_field(
            "groupsWithIncorrectOutputRecords",
            &self.groups_with_incorrect_output_records,
        )?;
        s.end()
    }
}

impl HealthCheckReport {
    pub(super) fn new_from_report(r: &AggregationReport) -> Self {
        let mut accounts_count_across_llgs = 0;
        let mut cashflows_count_across_llgs = 0;
        let mut total_principal_amount_across_llgs = 0.0;
        let mut total_interest_amount_across_llgs = 0.0;
        let mut llgs_with_mismatched_principal_amount_group_distribution = Vec::new();
        let mut llgs_with_mismatched_interest_amount_group_distribution = Vec::new();
        let mut llgs_with_summary_mismatches = HashMap::new();

        for (llg, report) in &r.llgs_report.report {
            // For comparison against the input
            accounts_count_across_llgs += report.accounts_total_report.accounts_count;
            cashflows_count_across_llgs += report.accounts_total_report.cashflows_count;
            total_principal_amount_across_llgs +=
                report.accounts_total_report.total_principal_amount;
            total_interest_amount_across_llgs += report.accounts_total_report.total_interest_amount;

            // An LLG's total vs. what is distributed within its groups
            let mut principal_amount_across_groups = 0.0;
            let mut interest_amount_across_groups = 0.0;
            for file_amount in &report.six_file_amounts {
                principal_amount_across_groups += file_amount.principal_amount;
                interest_amount_across_groups += file_amount.interest_amount;
            }
            if principal_amount_across_groups
                .approx_ne_ulps(&report.accounts_total_report.total_principal_amount, 2)
            {
                llgs_with_mismatched_principal_amount_group_distribution.push(llg.clone())
            }
            if interest_amount_across_groups
                .approx_ne_ulps(&report.accounts_total_report.total_interest_amount, 2)
            {
                llgs_with_mismatched_interest_amount_group_distribution.push(llg.clone())
            }

            // Summary Mismatches
            let amounts_in_summary = r
                .llg_summaries_report
                .report
                .get(llg)
                .expect("Error while getting llg.");
            let principal_amount_in_summary = amounts_in_summary.principal_amount;
            let interest_amount_in_summary = amounts_in_summary.interest_amount;
            if principal_amount_in_summary
                .approx_ne_ulps(&report.accounts_total_report.total_principal_amount, 2)
                || interest_amount_in_summary
                    .approx_ne_ulps(&report.accounts_total_report.total_interest_amount, 2)
            {
                let mut amount_differences_from_llg_to_summary =
                    PrincipalInterestAmountReport::new();
                amount_differences_from_llg_to_summary.interest_amount =
                    report.accounts_total_report.total_interest_amount - interest_amount_in_summary;
                amount_differences_from_llg_to_summary.principal_amount =
                    report.accounts_total_report.total_principal_amount
                        - principal_amount_in_summary;

                llgs_with_summary_mismatches
                    .insert(llg.clone(), amount_differences_from_llg_to_summary);
            }
        }

        let input_to_llgs_account_count_difference =
            accounts_count_across_llgs - (r.input_report.accounts_count * 2); // multiplied with 2 because every account generates 2 llg, this may change in future
        let input_to_llgs_cashflow_count_difference =
            cashflows_count_across_llgs - (r.input_report.cashflows_count * 2); // multiplied with 2 because every account generates 2 llg, this may change in future
        let input_to_llgs_principal_amount_difference =
            total_principal_amount_across_llgs - (r.input_report.total_principal_amount * 2.0); // multiplied with 2 because every account generates 2 llg, this may change in future
        let input_to_llgs_interest_amount_difference =
            total_interest_amount_across_llgs - (r.input_report.total_interest_amount * 2.0); // multiplied with 2 because every account generates 2 llg, this may change in future

        let mut groups_with_incorrect_output_records = Vec::new();
        let llgs_count = r.llgs_report.report.keys().count();

        // TODO: Bug. This will be more for the number of currencies
        let expected_output_records_count = llgs_count * 3; // Each LLG has 3 records written

        for (i, output_count) in r.output_records_report.report.iter().enumerate() {
            if *output_count as usize != expected_output_records_count {
                groups_with_incorrect_output_records.push(format!("group-{}", i))
            }
        }

        HealthCheckReport {
            input_to_llgs_account_count_difference,
            input_to_llgs_cashflow_count_difference,
            input_to_llgs_principal_amount_difference,
            input_to_llgs_interest_amount_difference,
            llgs_with_mismatched_principal_amount_file_distribution:
                llgs_with_mismatched_principal_amount_group_distribution,
            llgs_with_mismatched_interest_amount_file_distribution:
                llgs_with_mismatched_interest_amount_group_distribution,
            llgs_with_summary_mismatches,
            groups_with_incorrect_output_records,
        }
    }
}

fn llgs_to_string_vec(llgs_vec: &Vec<LLGKey>) -> Vec<String> {
    let mut string_vec = Vec::new();

    for llg in llgs_vec {
        string_vec.push(format!("{}", llg))
    }

    string_vec
}

// TODO: Remove this garbage once `LLGKey` can be successfully serialized to JSON at the key.
fn llg_map_to_string_map(
    m: &HashMap<LLGKey, PrincipalInterestAmountReport>,
) -> HashMap<String, PrincipalInterestAmountReport> {
    let mut string_map = HashMap::new();
    for (llg, amount) in m {
        string_map.insert(format!("{}", llg), amount.clone());
    }

    string_map
}
