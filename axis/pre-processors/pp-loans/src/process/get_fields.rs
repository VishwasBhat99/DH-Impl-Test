use process::structs::*;
use rbdate::DateParser;

pub fn get_gam_fields(new_acc: &mut LoanAccount, input_fields: Vec<&str>) {
    let date_parser: DateParser = DateParser::new("%d-%m-%Y".to_string(), true);
    let clr_bal_amt = input_fields[3].parse::<f64>().unwrap_or(0.0);
    let un_clr_bal_amt = input_fields[4].parse::<f64>().unwrap_or(0.0);
    let out_bal_amt = clr_bal_amt + un_clr_bal_amt;
    new_acc.acid = input_fields[0].to_string();
    new_acc.acct_crncy_code = input_fields[17].to_string();
    new_acc.cust_id = input_fields[6].to_string();
    new_acc.custname = input_fields[6].to_string();
    new_acc.acct_opn_date = date_parser.parse(input_fields[20]);
    new_acc.bacid = input_fields[2].to_string();
    new_acc.clr_bal_amt = clr_bal_amt;
    new_acc.foracid = input_fields[1].to_string();
    new_acc.gl_sub_head_code = input_fields[13].to_string();
    new_acc.out_bal_amt = out_bal_amt;
    new_acc.sanct_lim = input_fields[12].to_string().parse::<f64>().unwrap_or(0.0);
    new_acc.schm_code = input_fields[14].to_string();
    new_acc.schm_type = input_fields[15].to_string();
    new_acc.sol_id = input_fields[5].to_string();
    new_acc.segment_code = input_fields[55].to_string();
    new_acc.exrate = input_fields[53].parse::<f64>().unwrap_or(0.0);
}
