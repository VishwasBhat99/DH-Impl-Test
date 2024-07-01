///Input-Output Data Format  
#[derive(Debug, Clone, Default)]
pub struct Account {
    pub asondate: String,
    pub deal_number: String,
    pub deal_leg_number: String,
    pub deal_date: String,
    pub deal_type: String,
    pub fo_deal_number: i64,
    pub transaction_type: String,
    pub portfolio: String,
    pub counter_party: String,
    pub counterparty_category: String,
    pub deliver_flag: String,
    pub value_date: String,
    pub deal_curr_code: String,
    pub agnst_curr_code: String,
    pub deal_rate: f64,
    pub deal_curr_amount: f64,
    pub agnst_curr_amount: f64,
    pub reval_rate: String,
    pub pl_amount: f64,
    pub mduration: String,
    pub treasuryglcode: i64,

    //Fields that would be stamped only in output
    pub cf_type: String,
    pub cf_sub_type: String,
    pub cf_int_amount: f64,
    pub cf_prin_amount: f64,
    pub currency: String,
    pub cf_date: String,
}

impl Account {
    pub fn new(input_file: &str, inp_acc: &[&str], row: usize) -> Account {
        Account {
            asondate: get_str(input_file, inp_acc, 0, row),
            deal_number: get_str(input_file, inp_acc, 1, row),
            deal_leg_number: get_str(input_file, inp_acc, 2, row),
            deal_date: get_str(input_file, inp_acc, 3, row),
            deal_type: get_str(input_file, inp_acc, 4, row),
            fo_deal_number: get_str(input_file, inp_acc, 5, row)
                .parse::<i64>()
                .unwrap_or(0),
            transaction_type: get_str(input_file, inp_acc, 6, row),
            portfolio: get_str(input_file, inp_acc, 7, row),
            counter_party: get_str(input_file, inp_acc, 8, row),
            counterparty_category: get_str(input_file, inp_acc, 9, row),
            deliver_flag: get_str(input_file, inp_acc, 10, row),
            value_date: get_str(input_file, inp_acc, 11, row),
            deal_curr_code: get_str(input_file, inp_acc, 12, row),
            agnst_curr_code: get_str(input_file, inp_acc, 13, row),
            deal_rate: get_str(input_file, inp_acc, 14, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            deal_curr_amount: get_str(input_file, inp_acc, 15, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            agnst_curr_amount: get_str(input_file, inp_acc, 16, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            reval_rate: get_str(input_file, inp_acc, 17, row),
            pl_amount: get_str(input_file, inp_acc, 18, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            mduration: get_str(input_file, inp_acc, 19, row),
            treasuryglcode: get_str(input_file, inp_acc, 20, row)
                .parse::<i64>()
                .unwrap_or(0),

            //Default Values
            cf_type: "NA".to_string(),
            cf_sub_type: get_str(input_file, inp_acc, 6, row),
            cf_int_amount: get_str(input_file, inp_acc, 15, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            cf_prin_amount: get_str(input_file, inp_acc, 15, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            currency: get_str(input_file, inp_acc, 12, row),
            cf_date: get_str(input_file, inp_acc, 0, row),
        }
    }
}

///Cashflows Data Format  
#[derive(Debug, Clone)]
pub struct Cashflow {
    pub asondate: String,
    pub deal_num: String,
    pub cf_type: String,
    pub cf_sub_type: String,
    pub cf_amount: f64,
    pub currency: String,
    pub cf_date: String,
}

impl Cashflow {
    pub fn new(cf_file: &str, cf_acc: &[&str], row: usize) -> Cashflow {
        Cashflow {
            asondate: get_str(cf_file, cf_acc, 0, row),
            deal_num: get_str(cf_file, cf_acc, 1, row),
            cf_type: get_str(cf_file, cf_acc, 2, row),
            cf_sub_type: get_str(cf_file, cf_acc, 3, row),
            cf_amount: get_str(cf_file, cf_acc, 4, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            currency: get_str(cf_file, cf_acc, 5, row),
            cf_date: get_str(cf_file, cf_acc, 6, row),
        }
    }
}

pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}

pub fn format_output(output_rec: Account) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        output_rec.asondate,
        output_rec.deal_number,
        output_rec.deal_leg_number,
        output_rec.deal_date,
        output_rec.deal_type,
        output_rec.fo_deal_number,
        output_rec.transaction_type,
        output_rec.portfolio,
        output_rec.counter_party,
        output_rec.counterparty_category,
        output_rec.deliver_flag,
        output_rec.value_date,
        output_rec.deal_curr_code,
        output_rec.agnst_curr_code,
        output_rec.deal_rate,
        output_rec.deal_curr_amount,
        output_rec.agnst_curr_amount,
        output_rec.reval_rate,
        output_rec.pl_amount,
        output_rec.mduration,
        output_rec.treasuryglcode,
        output_rec.cf_type,
        output_rec.cf_sub_type,
        output_rec.cf_int_amount,
        output_rec.cf_prin_amount,
        output_rec.currency,
        output_rec.cf_date,
    )
}
