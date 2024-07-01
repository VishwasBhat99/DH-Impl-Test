#[derive(Debug, Clone)]
pub struct AccFieldNames {
    pub account_id: String,
    pub currency: String,
    pub int_rate: String,
    pub outstanding_bal: String,
    pub field_type: String,
    pub cf_principal_amount: String,
    pub principal: String,
    pub interest: String,
    pub cf_date: String,
    pub gl: String,
    pub start_date: String,
    pub maturity_date: String,
    pub rate_flag: String,
    pub branch: String,
    pub customer_id: String,
    pub customer_type: String,
    pub product_code: String,
    pub group: String,
    pub acc_branch: String,
    pub acc_number: String,
    pub acc_suffix: String,
    pub acc_type: String,
    pub deal_type: String,
    pub repricing_frequency: String,
    pub last_repr_date: String,
    pub next_repr_date: String,
    pub int_compounding_frequency: String,
    pub int_repayment_frequency: String,
    pub margin_rate: String,
    pub cpas: String,
    pub cust_constitution_code: String,
    pub customer_rating: String,
    pub p2: String,
    pub analysis_code: String,
    pub sundry_analysis_code: String,
    pub numeric_analysis_code: String,
    pub base_rate_code: String,
    pub differential_rate_code: String,
    pub accrued_int_amt: String,
    pub next_rollover_date: String,
    pub npa_flag: String,
    pub npa_type: String,
    pub rm: String,
    pub customer_name: String,
    pub monthly_avg_bal: String,
    pub pension_account_flag: String,
    pub waiver_flag: String,
    pub aorl: String,
    pub rl1: String,
    pub rl2: String,
    pub rl3: String,
    pub total_interest_amount: String,
    pub total_principal_amount: String,
    pub cashflows: String,
}

impl AccFieldNames {
    pub fn get_input_fields_names() -> AccFieldNames {
        AccFieldNames {
            account_id: "account_id".to_string(),
            currency: "currency".to_string(),
            int_rate: "int_rate".to_string(),
            outstanding_bal: "outstanding_bal".to_string(),
            field_type: "field_type".to_string(),
            cf_principal_amount: "cf_principal_amount".to_string(),
            principal: "principal".to_string(),
            interest: "interest".to_string(),
            cf_date: "cf_date".to_string(),
            gl: "gl".to_string(),
            start_date: "start_date".to_string(),
            maturity_date: "maturity_date".to_string(),
            rate_flag: "rate_flag".to_string(),
            branch: "branch".to_string(),
            customer_id: "customer_id".to_string(),
            customer_type: "customer_type".to_string(),
            product_code: "product_code".to_string(),
            group: "group".to_string(),
            acc_branch: "acc_branch".to_string(),
            acc_number: "acc_number".to_string(),
            acc_suffix: "acc_suffix".to_string(),
            acc_type: "acc_type".to_string(),
            deal_type: "deal_type".to_string(),
            repricing_frequency: "repricing_frequency".to_string(),
            last_repr_date: "last_repr_date".to_string(),
            next_repr_date: "next_repr_date".to_string(),
            int_compounding_frequency: "int_compounding_frequency".to_string(),
            int_repayment_frequency: "int_repayment_frequency".to_string(),
            margin_rate: "margin_rate".to_string(),
            cpas: "cpas".to_string(),
            cust_constitution_code: "cust_constitution_code".to_string(),
            customer_rating: "customer_rating".to_string(),
            p2: "p2".to_string(),
            analysis_code: "analysis_code".to_string(),
            sundry_analysis_code: "sundry_analysis_code".to_string(),
            numeric_analysis_code: "numeric_analysis_code".to_string(),
            base_rate_code: "base_rate_code".to_string(),
            differential_rate_code: "differential_rate_code".to_string(),
            accrued_int_amt: "accrued_int_amt".to_string(),
            next_rollover_date: "next_rollover_date".to_string(),
            npa_flag: "npa_flag".to_string(),
            npa_type: "npa_type".to_string(),
            rm: "rm".to_string(),
            customer_name: "customer_name".to_string(),
            monthly_avg_bal: "monthly_avg_bal".to_string(),
            pension_account_flag: "pension_account_flag".to_string(),
            waiver_flag: "waiver_flag".to_string(),
            aorl: "aorl".to_string(),
            rl1: "rl1".to_string(),
            rl2: "rl2".to_string(),
            rl3: "rl3".to_string(),
            total_interest_amount: "total_interest_amount".to_string(),
            total_principal_amount: "total_principal_amount".to_string(),
            cashflows: "cashflows".to_string(),
        }
    }
}
