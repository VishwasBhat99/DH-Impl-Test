use super::input_account::{BillSCFM, BillsGL};

pub fn get_op_line(scfm_data: &BillSCFM, gl_data: &BillsGL) -> String {
    let op_str = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        &scfm_data.invoice_no,
        &scfm_data.owner_id,
        &scfm_data.owner_name,
        &scfm_data.bills_os_id,
        &scfm_data.currency,
        if &scfm_data.npa_classification == &"0".to_string() {&scfm_data.bills_outstanding} else {&scfm_data.npa_amount},
        &scfm_data.acct_open_date,
        &scfm_data.maturity_date,
        &scfm_data.interest_type,
        &scfm_data.roi,
        &scfm_data.next_repricing_date,
        &scfm_data.last_repricing_date,
        &scfm_data.repricing_frequency,
        &scfm_data.benchmark,
        &scfm_data.npa_classification,
        &scfm_data.cust_classification,
        &scfm_data.gl_code,
        &scfm_data.constitution,
        &scfm_data.segment_code,
        &scfm_data.npa_amount,
        &scfm_data.scfm_foracid,
        gl_data.gl_sub_head_code,
        gl_data.foracid,
        gl_data.cust_id,
        gl_data.schm_code,
        gl_data.schm_type,
        gl_data.clr_bal_amt,
        gl_data.un_clr_bal_amt,
        gl_data.acct_crncy_code
    );
    op_str
}
