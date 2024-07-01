use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccFieldNames {
    pub account_number: String,
    pub curr_code: String,
    pub intt_rate: String,
    pub product_code: String,
    pub mat_date: String,
    pub rate_flag: String,
    pub repricing_index: String,
    pub psl: String,
    pub npa: String,
    pub indv_corp_flag: String,
    pub customer_type: String,
    pub asset_class_id: String,
    pub customer_id: String,
    pub prod_type: String,
    pub final_int_rate: String,
    pub cost_centre: String,
    pub alm_line: String,
    pub coa: String,
    pub division: String,
    pub rep_freq: String,
    pub next_repricing_date: String,
    pub last_repricing_date: String,
    pub asset_class: String,
    pub value_date: String,
    pub branch: String,
    pub org_tenor: String,
    pub rep_tenor: String,
    pub weaker: String,
    pub current_book_bal: String,
    pub customer_name: String,
    pub orig_bal: String,
    pub gr_ofs_gl: String,
    pub scheme_id: String,
    pub is_ofs_gl: String,
    pub re_ofs_gl: String,
    pub ui_ofs_gl: String,
    pub gr_dr: String,
    pub gr_cr: String,
    pub re_dr: String,
    pub re_cr: String,
    pub is_dr: String,
    pub is_cr: String,
    pub ui_dr: String,
    pub ui_cr: String,
    pub weaker_desc: String,
    pub int_income_gl: String,
    pub overdue_int_gl: String,
    pub int_on_cancellation_gl: String,
    pub writeoff_gl: String,
    pub int_income_gl_amt: String,
    pub overdue_int_gl_amt: String,
    pub int_on_cancellation_gl_amt: String,
    pub writeoff_gl_amt: String,
    pub cashflows: String
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(_path).expect("Unable to open Required-Fields-File");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string from Req-Fields File");
        let req_fields: AccFieldNames =
            serde_json::from_str(&buf[..]).expect("Required-Fields-File was not well-formatted");
        req_fields
    }

    pub fn get_input_fields_names(_path: &str) -> AccFieldNames {
        let req_fields = Self::new_from_path(_path);
        AccFieldNames {
            account_number: req_fields.account_number.to_string(),
            curr_code: req_fields.curr_code.to_string(),
            intt_rate: req_fields.intt_rate.to_string(),
            product_code: req_fields.product_code.to_string(),
            mat_date: req_fields.mat_date.to_string(),
            rate_flag: req_fields.rate_flag.to_string(),
            repricing_index: req_fields.repricing_index.to_string(),
            psl: req_fields.psl.to_string(),
            npa: req_fields.npa.to_string(),
            indv_corp_flag: req_fields.indv_corp_flag.to_string(),
            customer_type: req_fields.customer_type.to_string(),
            asset_class_id: req_fields.asset_class_id.to_string(),
            customer_id: req_fields.customer_id.to_string(),
            prod_type: req_fields.prod_type.to_string(),
            final_int_rate: req_fields.final_int_rate.to_string(),
            cost_centre: req_fields.cost_centre.to_string(),
            alm_line: req_fields.alm_line.to_string(),
            coa: req_fields.coa.to_string(),
            division: req_fields.division.to_string(),
            rep_freq: req_fields.rep_freq.to_string(),
            next_repricing_date: req_fields.next_repricing_date.to_string(),
            last_repricing_date: req_fields.last_repricing_date.to_string(),
            asset_class: req_fields.asset_class.to_string(),
            value_date: req_fields.value_date.to_string(),
            branch: req_fields.branch.to_string(),
            org_tenor: req_fields.org_tenor.to_string(),
            rep_tenor: req_fields.rep_tenor.to_string(),
            weaker: req_fields.weaker.to_string(),
            current_book_bal: req_fields.current_book_bal.to_string(),
            customer_name: req_fields.customer_name.to_string(),
            orig_bal: req_fields.orig_bal.to_string(),
            gr_ofs_gl: req_fields.gr_ofs_gl.to_string(),
            scheme_id: req_fields.scheme_id.to_string(),
            is_ofs_gl: req_fields.is_ofs_gl.to_string(),
            re_ofs_gl: req_fields.re_ofs_gl.to_string(),
            ui_ofs_gl: req_fields.ui_ofs_gl.to_string(),
            gr_dr: req_fields.gr_dr.to_string(),
            gr_cr: req_fields.gr_cr.to_string(),
            re_dr: req_fields.re_dr.to_string(),
            re_cr: req_fields.re_cr.to_string(),
            is_dr: req_fields.is_dr.to_string(),
            is_cr: req_fields.is_cr.to_string(),
            ui_dr: req_fields.ui_dr.to_string(),
            ui_cr: req_fields.ui_cr.to_string(),
            weaker_desc: req_fields.weaker_desc.to_string(),
            int_income_gl: req_fields.int_income_gl.to_string(),
            overdue_int_gl: req_fields.overdue_int_gl.to_string(),
            int_on_cancellation_gl: req_fields.int_on_cancellation_gl.to_string(),
            writeoff_gl: req_fields.writeoff_gl.to_string(),
            int_income_gl_amt: req_fields.int_income_gl_amt.to_string(),
            overdue_int_gl_amt: req_fields.overdue_int_gl_amt.to_string(),
            int_on_cancellation_gl_amt: req_fields.int_on_cancellation_gl_amt.to_string(),
            writeoff_gl_amt: req_fields.writeoff_gl_amt.to_string(),
            cashflows: req_fields.cashflows.to_string(),
        }
    }
}
