#[derive(Debug, Clone)]
/// The structure in which the input-account is expected with '|' as a separator
pub struct InputAccount {
    pub issuer_number: String,
    pub instrument_type: String,
    pub isin: String,
    pub lender_name: String,
    pub outstanding_bal: f64,
    pub currency: String,
    pub issue_date: String,
    pub mat_date: String,
    pub coupon_rate: f64,
    pub coupon_type: String,
    pub benchmark: String,
    pub coupon_freq: String,
    pub next_coupon_date: String,
    pub next_repricing_date: String,
    pub repricing_freq: String,
    pub int_calc_basis: String,
    pub acquisition_val: f64,
    pub next_in_payout_date: String,
    pub prod_type: String,
}

impl InputAccount {
    pub fn new(input_acc: Vec<&str>) -> InputAccount {
        InputAccount {
            issuer_number: input_acc[0].to_string(),
            instrument_type: input_acc[1].to_string(),
            isin: input_acc[2].to_string(),
            lender_name: input_acc[3].to_string(),
            outstanding_bal: input_acc[4].to_string().parse::<f64>().unwrap_or(0.0),
            currency: input_acc[5].to_string(),
            issue_date: input_acc[6].to_string(),
            mat_date: input_acc[7].to_string(),
            coupon_rate: input_acc[8].to_string().parse::<f64>().unwrap_or(0.0),
            coupon_type: input_acc[9].to_string(),
            benchmark: input_acc[10].to_string(),
            coupon_freq: input_acc[11].to_string(),
            next_coupon_date: input_acc[12].to_string(),
            next_repricing_date: input_acc[13].to_string(),
            repricing_freq: input_acc[14].to_string(),
            int_calc_basis: input_acc[15].to_string(),
            acquisition_val: input_acc[16].to_string().parse::<f64>().unwrap_or(0.0),
            next_in_payout_date: input_acc[17].to_string(),
            prod_type: input_acc[18].to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which the cashflows for each account is expected with '|' as a separator
pub struct Cashflow {
    pub issuer_number: i64,
    pub security_number: i64,
    pub cashflow_type: String,
    pub cashflow_amount: f64,
    pub cashflow_date: String,
}

impl Cashflow {
    pub fn new(cashflow: Vec<&str>) -> Cashflow {
        Cashflow {
            issuer_number: cashflow[0].to_string().parse::<i64>().unwrap_or(0),
            security_number: cashflow[1].to_string().parse::<i64>().unwrap_or(0),
            cashflow_type: cashflow[2].to_string(),
            cashflow_amount: cashflow[3].to_string().parse::<f64>().unwrap_or(0.0),
            cashflow_date: cashflow[4].to_string(),
        }
    }
    pub fn def() -> Vec<Cashflow> {
        ::std::default::Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which the benpos-data for each isin is expected in a excel file
pub struct BenposData {
    pub isin: String,
    pub name1: String,
    pub pangir1: String,
    pub position: f64,
    pub nfacevn: f64,
    pub cmdate: String,
}

impl BenposData {
    pub fn new(benpos_data: &[calamine::DataType]) -> BenposData {
        BenposData {
            isin: benpos_data[58].to_string(),
            name1: benpos_data[15].to_string(),
            pangir1: benpos_data[28].to_string(),
            position: benpos_data[51].to_string().parse::<f64>().unwrap_or(0.0),
            nfacevn: benpos_data[67].to_string().parse::<f64>().unwrap_or(0.0),
            cmdate: benpos_data[70].to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which the benpos-map-data for each pangir1 is expected in a excel file
pub struct BenposMap {
    pub pangir1: String,
    pub short_name: String,
    pub name1: String,
    pub category: String,
}

impl BenposMap {
    pub fn new(benpos_map: &[calamine::DataType]) -> BenposMap {
        BenposMap {
            pangir1: benpos_map[0].to_string(),
            short_name: benpos_map[1].to_string(),
            name1: benpos_map[2].to_string(),
            category: benpos_map[3].to_string(),
        }
    }
    pub fn def() -> BenposMap {
        ::std::default::Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which the float-map-data for each isin is expected in a excel file
pub struct FloatMap {
    pub isin: String,
    pub reset_date: String,
    pub amt: f64,
    pub description: String,
}

impl FloatMap {
    pub fn new(float_map: &[calamine::DataType]) -> FloatMap {
        FloatMap {
            isin: float_map[0].to_string(),
            reset_date: float_map[1].to_string(),
            amt: float_map[2].to_string().parse::<f64>().unwrap_or(0.0),
            description: float_map[3].to_string(),
        }
    }
    pub fn def() -> FloatMap {
        ::std::default::Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which the Borrowing UpdateType Master Data and is expected in a excel file
pub struct BorrUpdateMap {
    pub update_type: String,
    pub cf_type: String,
    pub sign: String,
}

impl BorrUpdateMap {
    pub fn new(borr_update_type_map: &[calamine::DataType]) -> BorrUpdateMap {
        BorrUpdateMap {
            update_type: borr_update_type_map[0].to_string(),
            cf_type: borr_update_type_map[1].to_string(),
            sign: borr_update_type_map[2].to_string(),
        }
    }
    pub fn def() -> BorrUpdateMap {
        ::std::default::Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which output is written
pub struct OutputAccount {
    pub cust_no: String,
    pub reference: String,
    pub cust_name: String,
    pub branch_cd: String,
    pub norm_int_rt: String,
    pub acurl_freq: String,
    pub book_dt: String,
    pub val_dt: String,
    pub mat_dt: String,
    pub due_dt: String,
    pub user_def_stats: String,
    pub prod_cd: String,
    pub gl: String,
    pub curr: String,
    pub prin_ost_bal: String,
    pub component: String,
    pub amt_due: String,
    pub amt_setld: String,
    pub cf_amt: String,
    pub spread: String,
    pub bucket_category: String,
    pub is_secured: String,
    pub product_type: String,
    pub composition_percentage: String,
    pub old_rt_typ: String,
    pub old_benchmark: String,
    pub nxt_call_dt: String,
    pub nxt_put_dt: String,
    pub rt_flag_new: String,
    pub rt_cd_new: String,
    pub ucid: String,
    pub alm_line: String,
    pub ia_llg: String,
    pub balm_llg: String,
    pub coupon_freq: String,
    pub nxt_repricing_dt: String,
    pub lst_repricing_dt: String,
    pub as_on_dt: String,
    pub int_basis: String,
    pub int_calc_typ: String,
    pub cust_typ: String,
    pub npa_typ: String,
    pub bmid: String,
    pub division: String,
}

impl OutputAccount {
    pub fn new() -> OutputAccount {
        ::std::default::Default::default()
    }
}

pub fn format_output(output_rec: &OutputAccount) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        output_rec.cust_no,
        output_rec.reference,
        output_rec.cust_name,
        output_rec.branch_cd,
        output_rec.norm_int_rt,
        output_rec.acurl_freq,
        output_rec.book_dt,
        output_rec.val_dt,
        output_rec.mat_dt,
        output_rec.due_dt,
        output_rec.user_def_stats,
        output_rec.prod_cd,
        output_rec.gl,
        output_rec.curr,
        output_rec.prin_ost_bal,
        output_rec.component,
        output_rec.amt_due,
        output_rec.amt_setld,
        output_rec.cf_amt,
        output_rec.spread,
        output_rec.bucket_category,
        output_rec.is_secured,
        output_rec.product_type,
        output_rec.composition_percentage,
        output_rec.old_rt_typ,
        output_rec.old_benchmark,
        output_rec.nxt_call_dt,
        output_rec.nxt_put_dt,
        output_rec.rt_flag_new,
        output_rec.rt_cd_new,
        output_rec.ucid,
        output_rec.alm_line,
        output_rec.ia_llg,
        output_rec.balm_llg,
        output_rec.coupon_freq,
        output_rec.nxt_repricing_dt,
        output_rec.lst_repricing_dt,
        output_rec.as_on_dt,
        output_rec.int_basis,
        output_rec.int_calc_typ,
        output_rec.cust_typ,
        output_rec.npa_typ,
        output_rec.bmid,
        output_rec.division,
    )
}
