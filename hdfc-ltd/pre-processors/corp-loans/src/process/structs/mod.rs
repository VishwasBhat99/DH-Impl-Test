use rbdate::{DateParser, NaiveDate};
use std::f64::consts;

#[derive(Debug, Clone)]
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
    pub compmis1: String,
    pub compmis2: String,
    pub compmis3: String,
    pub old_rt_cd: String,
    pub old_rt_typ: String,
    pub old_benchmark: String,
    pub nxt_reset_dt: String,
    pub last_reset_dt: String,
    pub rt_flag_new: String,
    pub rt_cd_new: String,
    pub division: String,
    pub alm_line: String,
    pub ia_llg: String,
    pub balm_llg: String,
    pub repricing_freq: String,
    pub nxt_repricing_dt: String,
    pub lst_repricing_dt: String,
    pub as_on_dt: String,
    pub int_basis: String,
    pub int_calc_typ: String,
    pub cust_typ: String,
    pub npa_typ: String,
    pub bmid: String,
    pub pre_emi_ovd_amt: String,
    pub exc_pre_emi_amt: String,
    pub emi_ovd_amt: String,
    pub exc_emi_amt: String,
    pub pre_emi_ovd_gl_cd: String,
    pub exc_pre_emi_gl_cd: String,
    pub emi_ovd_gl_cd: String,
    pub exc_emi_gl_cd: String,
}

pub fn format_op_rec(data: &OutputAccount) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        data.cust_no,
        data.reference,
        data.cust_name,
        data.branch_cd,
        data.norm_int_rt,
        data.acurl_freq,
        data.book_dt,
        data.val_dt,
        data.mat_dt,
        data.due_dt,
        data.user_def_stats,
        data.prod_cd,
        data.gl,
        data.curr,
        data.prin_ost_bal,
        data.component,
        data.amt_due,
        data.amt_setld,
        data.cf_amt,
        data.spread,
        data.compmis1,
        data.compmis2,
        data.compmis3,
        data.old_rt_cd,
        data.old_rt_typ,
        data.old_benchmark,
        data.nxt_reset_dt,
        data.last_reset_dt,
        data.rt_flag_new,
        data.rt_cd_new,
        data.division,
        data.alm_line,
        data.ia_llg,
        data.balm_llg,
        data.repricing_freq,
        data.nxt_repricing_dt,
        data.lst_repricing_dt,
        data.as_on_dt,
        data.int_basis,
        data.int_calc_typ,
        data.cust_typ,
        data.npa_typ,
        data.bmid,
        data.pre_emi_ovd_amt,
        data.exc_pre_emi_amt,
        data.emi_ovd_amt,
        data.exc_emi_amt,
        data.pre_emi_ovd_gl_cd,
        data.exc_pre_emi_gl_cd,
        data.emi_ovd_gl_cd,
        data.exc_emi_gl_cd,
    )
}

pub fn get_op_data(
    acc_fields: Vec<String>,
    as_on_dt: &NaiveDate,
    lm_alm_line: String,
) -> OutputAccount {
    // Default cf type
    let mut cf_type = "PRINCIPAL";
    let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);
    let book_dt: NaiveDate = date_parser.parse_opt(&acc_fields[11]).unwrap_or(*as_on_dt);
    let val_dt: NaiveDate = date_parser.parse_opt(&acc_fields[11]).unwrap_or(*as_on_dt);
    let mat_dt: NaiveDate = match date_parser.parse_opt(&acc_fields[12]) {
        Some(date) => date,
        None => {
            // Since maturity date is not present
            cf_type = "FROZEN";
            as_on_dt.succ()
        }
    };
    let nxt_reset_dt: NaiveDate = date_parser
        .parse_opt(&acc_fields[19])
        .unwrap_or(NaiveDate::from_ymd(2099, 12, 31));
    let last_reset_dt: NaiveDate = date_parser.parse_opt(&acc_fields[18]).unwrap_or(val_dt);
    let nxt_repricing_dt: NaiveDate = date_parser
        .parse_opt(&acc_fields[19])
        .unwrap_or(NaiveDate::from_ymd(2099, 12, 31));
    let lst_repricing_dt: NaiveDate = date_parser.parse_opt(&acc_fields[18]).unwrap_or(val_dt);
    // Start ---calculation of #2384 new fields
    let pre_emi_ovd_amt;
    let exc_pre_emi_amt;
    let emi_ovd_amt;
    let exc_emi_amt;
    let cur_pmios: f64 = acc_fields[29].parse().unwrap_or(0.0);
    if cur_pmios >= 0.0 {
        pre_emi_ovd_amt = cur_pmios.abs().to_string();
    } else {
        pre_emi_ovd_amt = "".to_string();
    }
    if cur_pmios >= 0.0 {
        exc_pre_emi_amt = "".to_string();
    } else {
        exc_pre_emi_amt = cur_pmios.abs().to_string();
    }
    let cur_emios: f64 = acc_fields[30].parse().unwrap_or(0.0);
    if cur_emios >= 0.0 {
        emi_ovd_amt = cur_emios.abs().to_string();
    } else {
        emi_ovd_amt = "".to_string();
    }
    if cur_emios >= 0.0 {
        exc_emi_amt = "".to_string();
    } else {
        exc_emi_amt = cur_emios.abs().to_string();
    }
    let pre_emi_ovd_gl_cd = acc_fields[31].to_string();
    let exc_pre_emi_gl_cd = acc_fields[32].to_string();
    let emi_ovd_gl_cd = acc_fields[33].to_string();
    let exc_emi_gl_cd = acc_fields[34].to_string();
    //End ---calculation of #2384 new fields
    let data = OutputAccount {
        cust_no: acc_fields[3].to_string(),
        reference: acc_fields[0].to_string(),
        cust_name: acc_fields[1].to_string(),
        branch_cd: acc_fields[2].to_string(),
        norm_int_rt: acc_fields[14].to_string(),
        acurl_freq: acc_fields[31].to_string(),
        book_dt: book_dt.format("%d-%m-%Y").to_string(),
        val_dt: val_dt.format("%d-%m-%Y").to_string(),
        mat_dt: mat_dt.format("%d-%m-%Y").to_string(),
        due_dt: mat_dt.format("%d-%m-%Y").to_string(),
        user_def_stats: acc_fields[24].to_string(),
        prod_cd: acc_fields[6].to_string(),
        gl: acc_fields[5].to_string(),
        curr: acc_fields[10].to_string(),
        prin_ost_bal: acc_fields[9].to_string(),
        component: cf_type.to_string(),
        amt_due: "NA".to_string(),
        amt_setld: "NA".to_string(),
        cf_amt: acc_fields[9].to_string(),
        spread: acc_fields[17].to_string(),
        compmis1: "NA".to_string(),
        compmis2: "NA".to_string(),
        compmis3: "NA".to_string(),
        old_rt_cd: "NA".to_string(),
        old_rt_typ: "NA".to_string(),
        old_benchmark: acc_fields[16].to_string(),
        nxt_reset_dt: nxt_reset_dt.format("%d-%m-%Y").to_string(),
        last_reset_dt: last_reset_dt.format("%d-%m-%Y").to_string(),
        rt_flag_new: acc_fields[13].to_string(),
        rt_cd_new: acc_fields[16].to_string(),
        division: acc_fields[28].to_string(),
        alm_line: lm_alm_line,
        ia_llg: acc_fields[32].to_string(),
        balm_llg: acc_fields[33].to_string(),
        repricing_freq: acc_fields[20].to_string(),
        nxt_repricing_dt: nxt_repricing_dt.format("%d-%m-%Y").to_string(),
        lst_repricing_dt: lst_repricing_dt.format("%d-%m-%Y").to_string(),
        as_on_dt: as_on_dt.format("%d-%m-%Y").to_string(),
        int_basis: acc_fields[15].to_string(),
        int_calc_typ: "NA".to_string(),
        cust_typ: acc_fields[34].trim().to_string(),
        npa_typ: acc_fields[24].to_string(),
        bmid: acc_fields[16].to_string(),
        //add new fields
        pre_emi_ovd_amt,
        exc_pre_emi_amt,
        emi_ovd_amt,
        exc_emi_amt,
        pre_emi_ovd_gl_cd,
        exc_pre_emi_gl_cd,
        emi_ovd_gl_cd,
        exc_emi_gl_cd,
    };
    data
}
