use process::structs::*;
use process::HashMap;
use rbdate::{DateParser, NaiveDate};

pub fn get_loan_acct_fields(
    new_acc: &mut LoanAccount,
    input_fields: Vec<&str>,
    mclr_map: &HashMap<String, NaiveDate>,
    plr_data_map: &HashMap<String, String>,
    loan_repayment_str_map: &HashMap<String, LoanRepStr>,
    as_on_date: NaiveDate,
    npa_data: NpaData,
) {
    let date_parser: DateParser = DateParser::new("%d-%m-%Y".to_string(), true);
    let _clr_bal_amt = input_fields[3].parse::<f64>().unwrap_or(0.0);
    let _un_clr_bal_amt = input_fields[4].parse::<f64>().unwrap_or(0.0);
    new_acc.accid = input_fields[0].to_string();
    new_acc.sanct_limit = input_fields[1].trim().to_string();
    new_acc.pout_bal = if npa_data.npa_classification == "0" {
        input_fields[2].parse::<f64>().unwrap_or(0.0)
    } else {
        npa_data.npa_amount.parse::<f64>().unwrap_or(0.0)
    };
    new_acc.int_rate = input_fields[3].to_string();
    new_acc.ccy = input_fields[5].to_string();
    new_acc.npa_classification = npa_data.npa_classification;
    new_acc.final_npa_class = npa_data.final_npa_class;
    new_acc.sdate = if input_fields[6].trim() == "BLANKS"
        || input_fields[6].trim() == "Null"
        || input_fields[6].trim() == ""
    {
        as_on_date
    } else {
        date_parser.parse(input_fields[6].trim())
    };
    match input_fields[4].trim() {
        "Y" => {
            new_acc.ei_or_nonei = "EI".to_string();
            match mclr_map.get(input_fields[0]) {
                Some(val) => new_acc.repricing_date = *val,
                None => {}
            }
        }
        _ => {
            new_acc.ei_or_nonei = "NONEI".to_string();
            new_acc.floating_type = "BFLT".to_string();
        }
    };
    new_acc.dis_amt = input_fields[2].parse::<f64>().unwrap_or(0.0);
    new_acc.dis_date = if input_fields[9].trim() == "BLANKS"
        || input_fields[9].trim() == "Null"
        || input_fields[9].trim() == ""
    {
        as_on_date
    } else {
        date_parser.parse(input_fields[9].trim())
    };

    new_acc.scheme_code = input_fields[7].to_string();
    new_acc.cbs_gl_code = input_fields[7].to_string();
    new_acc.int_type = match input_fields[8].trim() {
        "FLT" => "Y".to_string(),
        _ => "N".to_string(),
    };
    new_acc.floating_type =
        if input_fields[8].trim() == "FLT" && mclr_map.contains_key(input_fields[0]) {
            "MCFLT".to_string()
        } else {
            "BFLT".to_string()
        };
    new_acc.last_installment_date = match input_fields[9].trim() {
        "BLANKS" | "Null" | "" => {
            if input_fields[6].trim() == "BLANKS"
                || input_fields[6].trim() == "Null"
                || input_fields[6].trim() == ""
            {
                as_on_date
            } else {
                date_parser.parse(input_fields[6].trim())
            }
        }
        _ => date_parser.parse(input_fields[9].trim()),
    };
    new_acc.frequency_type = match plr_data_map.get(&input_fields[0].to_string()) {
        Some(_acc_no) => "N1".to_string(),
        None => match input_fields[8] {
            "FLT" => "Y".to_string(),
            _ => "N".to_string(),
        },
    };
    let mut date_flag_exists = false;
    if input_fields[9].trim() == "BLANKS"
        || input_fields[9].trim() == "Null"
        || input_fields[9].trim() == ""
    {
        if input_fields[6].trim() == "BLANKS"
            || input_fields[6].trim() == "Null"
            || input_fields[6].trim() == ""
        {
            new_acc.pri_inst_start_date = as_on_date;
            new_acc.int_inst_start_date = as_on_date;
        } else {
            new_acc.pri_inst_start_date = date_parser.parse(input_fields[6]);
            new_acc.int_inst_start_date = date_parser.parse(input_fields[6]);
        }
    } else {
        date_flag_exists = true;
    }
    match loan_repayment_str_map.get(&input_fields[0].to_string()) {
        Some(loan_repayment_str) => {
            new_acc.total_num_inst = loan_repayment_str.total_num_inst;
            new_acc.num_inst_paid = loan_repayment_str.num_inst_paid;
            new_acc.inst_start_date = loan_repayment_str.inst_start_date;
            new_acc.num_inst = loan_repayment_str.total_num_inst - loan_repayment_str.num_inst_paid;
            new_acc.p_frequency_code = loan_repayment_str.freq_type.to_owned();
            new_acc.i_frequency_code = loan_repayment_str.freq_type.to_owned();
            new_acc.freq_period = loan_repayment_str.freq_period;
            new_acc.cfp_amt = loan_repayment_str.inst_amt;
            let frequency = match &loan_repayment_str.freq_type as &str {
                "M" => 1,
                "Q" => 3,
                "H" => 6,
                "Y" => 12,
                _ => 1,
            };
            new_acc.freq = frequency;
            if date_flag_exists {
                let inst_date = if input_fields[9].trim() == "BLANKS"
                    || input_fields[9].trim() == "Null"
                    || input_fields[9].trim() == ""
                {
                    as_on_date
                } else {
                    date_parser.parse(input_fields[9])
                };
                let pri_inst_start_date =
                    rbdate::increment_date_by_months(inst_date, frequency as u16);
                //   .unwrap_or_else(|| date_from_timestamp(0));
                let int_inst_start_date =
                    rbdate::increment_date_by_months(inst_date, frequency as u16);
                // .unwrap_or_else(|| date_from_timestamp(0));
                new_acc.pri_inst_start_date = pri_inst_start_date;
                new_acc.int_inst_start_date = int_inst_start_date;
            }
        }
        None => {
            if date_flag_exists {
                let inst_date = if input_fields[9].trim() == "BLANKS"
                    || input_fields[9].trim() == "Null"
                    || input_fields[9].trim() == ""
                {
                    as_on_date
                } else {
                    date_parser.parse(input_fields[9])
                };
                let pri_inst_start_date = rbdate::increment_date_by_months(inst_date, 1);
                //  .unwrap_or_else(|| date_from_timestamp(0));
                let int_inst_start_date = rbdate::increment_date_by_months(inst_date, 1);
                // .unwrap_or_else(|| date_from_timestamp(0));
                new_acc.pri_inst_start_date = pri_inst_start_date;
                new_acc.int_inst_start_date = int_inst_start_date;
            }
        }
    }
}

pub fn get_loan_rep_str_fields(
    input_field: &[&str],
    loan_repayment_str_map: &mut HashMap<String, LoanRepStr>,
    as_on_date: NaiveDate,
) {
    let date_parser: DateParser = DateParser::new("%d-%m-%Y".to_string(), true);
    let inst_st_dt = if input_field[6].trim() == "BLANKS"
        || input_field[6].trim() == "Null"
        || input_field[6].trim() == ""
    {
        as_on_date
    } else {
        date_parser.parse(input_field[6].trim())
    };
    loan_repayment_str_map.insert(
        input_field[0].to_string(),
        LoanRepStr {
            total_num_inst: input_field[1].parse::<i64>().unwrap_or(0),
            num_inst_paid: input_field[2].parse::<i64>().unwrap_or(0),
            inst_start_date: inst_st_dt,
            freq_type: input_field[3].trim().to_string(),
            freq_period: input_field[4].parse::<i64>().unwrap_or(0),
            inst_amt: input_field[5].parse::<f64>().unwrap_or(0.0),
        },
    );
}
