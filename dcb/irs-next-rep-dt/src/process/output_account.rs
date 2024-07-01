use rbdate::NaiveDate;

use super::input_account::InputData;

pub fn format_output(output_record: InputData) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        output_record.acid,
        output_record.foracid,
        output_record.sol_id,
        output_record.acct_opn_dt.format("%d-%m-%Y").to_string(),
        output_record.gl_sub_head_code,
        output_record.schm_code,
        output_record.schm_typ,
        output_record.acct_crncy_code,
        output_record.rep_shdl_num,
        output_record.rep_shdl_date.format("%d-%m-%Y").to_string(),
        output_record.dis_shdl_num,
        output_record.dis_shdl_date.format("%d-%m-%Y").to_string(),
        output_record.dis_amt,
        output_record.clr_bal_amt,
        output_record.sanct_lim,
        output_record.rephasement_principal,
        output_record.ei_perd_end_date.format("%d-%m-%Y").to_string(),
        output_record.cust_id,
        output_record.cust_name,
        output_record.ei_schm_flg,
        output_record.int_basis,
        output_record.ei_formula_flg,
        output_record.ei_intcalc_freq,
        output_record.ei_method,
        output_record.int_rate,
        output_record.int_type,
        output_record.next_rep_date.format("%d-%m-%Y").to_string(),
        output_record.last_rep_date.format("%d-%m-%Y").to_string(),
        output_record.rep_freq,
        output_record.float_rate_bench_mark,
        output_record.spread,
        output_record.npa_flag,
        output_record.npa_classification,
        output_record.npa_amt,
        output_record.cust_country_cd,
        output_record.cust_credit_rating,
        output_record.cust_industry_cd,
        output_record.cust_industry_cd,
        output_record.exchange_rt,
        output_record.custom1,
        output_record.custom2,
        output_record.custom3  
    )
}
pub fn get_writer(file_path: &str) -> std::io::BufWriter<std::fs::File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
