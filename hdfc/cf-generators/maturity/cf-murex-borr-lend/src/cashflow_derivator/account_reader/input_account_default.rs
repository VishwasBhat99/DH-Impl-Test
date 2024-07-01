use cashflow_derivator::account_reader::InputAccount;
use statics::*;

impl InputAccount {
    pub fn new() -> InputAccount {
        ::std::default::Default::default()
    }
}

impl Default for InputAccount {
    fn default() -> InputAccount {
        InputAccount {
            deal_id: String::default(),
            branch: String::default(),
            inst_name: String::default(),
            lend_borr_typ: String::default(),
            typology: String::default(),
            usage: String::default(),
            sub_typ_borr_lend: String::default(),
            cntrprty: String::default(),
            crtn_dt: None,
            val_date: None,
            deal_date: None,
            ccy: String::default(),
            crnt_deal_amt: DEFAULT_FLOAT,
            crnt_conv_rt_lcy: DEFAULT_FLOAT,
            crnt_deal_amt_lcy: DEFAULT_FLOAT,
            roi: DEFAULT_FLOAT,
            tenor_days: DEFAULT_INT,
            mat_dt: None,
            prin_amt: DEFAULT_FLOAT,
            int_amt: DEFAULT_FLOAT,
            cf_typ: String::default(),
            flow_typ: String::default(),
            mat_amt: DEFAULT_FLOAT,
            dealer_name: String::default(),
            nds_ref_no: String::default(),
            nxt_fix_dt: None,
            residual_tenor: DEFAULT_INT,
            nxt_put_dt: None,
            nxt_call_dt: None,
            nxt_int_pay_dt: None,
            int_pay_tenor: DEFAULT_INT,
            aip_air: DEFAULT_FLOAT,
            downgrade_clause: String::default(),
            avg_monthly_bal: String::default(),
            glcode: String::default(),
            cntrprty_ctgry_1: String::default(),
            cntrprty_ctgry_2: String::default(),
            cntrprty_ctgry_3: String::default(),
            cntrprty_ctgry_4: String::default(),
            int_pay_rec: String::default(),
            bckt_days: DEFAULT_INT,
            cntrprty_ctgry_5: String::default(),
            ind_outside_ind: String::default(),
            system_gl: String::default(),
            prod_concat: String::default(),
            alm_concat: String::default(),
            div: String::default(),
            alm_line: String::default(),
            ia_line: String::default(),
            sma_flag: String::default(),
        }
    }
}
