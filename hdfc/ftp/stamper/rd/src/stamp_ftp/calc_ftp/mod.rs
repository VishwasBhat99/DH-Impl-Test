use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::aggr_key::Customer;
use stamp_ftp::cfinput::AccFieldNames;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

mod append_output;

pub fn calc_ftp(
    acc_data_in: &mut AccountWithCFs,
    inputfieldnames: &AccFieldNames,
    log: &Logger,
    diag_log: &Logger,
    ftprunid: i64,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    agr_bal: &mut HashMap<Customer, f64>,
    avg_bal: &HashMap<String, f64>,
) -> (String, String) {
    //Margin Method
    let int_rate = acc_data_in
        .get_f64_for_key(&inputfieldnames.int_rt)
        .unwrap();
    let mut _lst_out: Vec<String> = Vec::new();
    let mut outstr: String = String::new();
    let ccy = acc_data_in
        .get_string_for_key(&inputfieldnames.ccy)
        .unwrap()
        .to_string();

    let (aggr_balance, isagr) = get_aggr_bal(agr_bal, acc_data_in, inputfieldnames);
    let (adjrate1, out_type) = get_dep_adj_rate(aggr_balance, acc_data_in, inputfieldnames);
    let base_rate = int_rate;
    let final_tpr = base_rate + adjrate1;
    let locked_spread = final_tpr - int_rate;

    if ccy == "INR" {
        outstr = append_output::form_out_data(
            acc_data_in,
            inputfieldnames,
            base_rate,
            adjrate1,
            final_tpr,
            locked_spread,
            aggr_balance,
            ftprunid,
            from_date,
            to_date,
            avg_bal,
            log,
            diag_log,
        );
    } else {
        outstr = append_output::form_out_data(
            acc_data_in,
            inputfieldnames,
            base_rate,
            1.0,
            base_rate + 1.0,
            1.0,
            aggr_balance,
            ftprunid,
            from_date,
            to_date,
            avg_bal,
            log,
            diag_log,
        );
    }

    outstr.push('\n');
    (outstr, out_type)
}

pub fn get_dep_adj_rate(
    aggr_bal: f64,
    acc_data_in: &mut AccountWithCFs,
    inputfieldnames: &AccFieldNames,
) -> (f64, String) {
    let mut adj_rate: f64 = 0.0;
    let mut out_type: String = String::new();

    if aggr_bal >= 20000000.00 && aggr_bal < 50000000.00 {
        adj_rate = 0.5;
        out_type = "two_to_five".to_string();
    } else if aggr_bal >= 50000000.00 {
        adj_rate = 0.25;
        out_type = "greater_than_five".to_string();
    } else {
        adj_rate = 1.0;
        out_type = "less_than_two".to_string();
    }

    (adj_rate, out_type)
}

pub fn get_aggr_bal(
    aggr_bal: &mut HashMap<Customer, f64>,
    acc_data_in: &mut AccountWithCFs,
    inputfieldnames: &AccFieldNames,
) -> (f64, bool) {
    let mut isagr: bool = true;
    let cust_id = acc_data_in
        .get_i64_for_key(&inputfieldnames.cust_id)
        .unwrap();
    let ccy = acc_data_in
        .get_string_for_key(&inputfieldnames.ccy)
        .unwrap()
        .to_string();
    let st_dt = acc_data_in.get_i64_for_key(&inputfieldnames.st_dt).unwrap();
    let mat_dt = acc_data_in
        .get_i64_for_key(&inputfieldnames.mat_dt)
        .unwrap();

    let acc_bal = acc_data_in.get_f64_for_key(&inputfieldnames.amt).unwrap();

    let cust_key = Customer::new(cust_id, st_dt, mat_dt, ccy);

    let balance = match aggr_bal.get(&cust_key) {
        Some(x) => *x,
        None => {
            isagr = false;
            acc_bal
        }
    };

    (balance, isagr)
}

pub fn write_out_text(path: &str, _lst_out: Vec<String>) {
    let full_file_path = format!("{}.txt", path);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(full_file_path)
        .unwrap();

    for i in &_lst_out {
        writeln!(file, "{}", i);
    }
}
