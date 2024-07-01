use super::FTPDates;
use super::{gls::Gls, InputAccount, OutputLines};
use macros;
use rbdate::{increment_date_by_months, num_days_start_to_end, NaiveDate};
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(
    acc: &InputAccount,
    t_ora_mis1: &mut HashMap<String, String>,
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, String>,
    t_ora_cat: &mut HashMap<String, String>,
    gls: &Gls,
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    ia_line: &mut HashMap<String, String>,
    prin_amt: f64,
    int_amt: f64,
    ost_bal: f64,
    fb_master: &mut HashMap<String, FTPDates>,
    def_date: NaiveDate,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> OutputLines {
    let mut op_lines = OutputLines::new();

    let fv_gl = gls.fv_gl.to_string();
    let ora_mis1 = t_ora_mis1
        .entry(fv_gl.to_string())
        .or_insert_with(|| "".to_string());
    let alm_concat = get_alm_concat(
        ora_mis1,
        t_ora_prod
            .entry(fv_gl.to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_gl
            .entry(fv_gl.to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_cat
            .entry(fv_gl.to_string())
            .or_insert_with(|| "".to_string()),
    );

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let ia_line = ia_line
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let div = div
        .entry(ora_mis1.to_string())
        .or_insert_with(|| "".to_string());

    log_debug!(
        log,
        "account: `{}`, gl: `{}`, alm_concat: `{}`, alm_line: `{}`, ia_line: `{}`, div: `{}`.",
        acc.deal_no,
        acc.gl_cd,
        alm_concat,
        alm_line,
        ia_line,
        div
    );
    let ftp_dates = match fb_master.get(&acc.isin) {
        Some(dates) => *dates,
        None => FTPDates {
            lst_rep_dt: def_date,
            nxt_rep_dt: def_date,
        },
    };

    op_lines.processed_lines = get_line(
        acc,
        as_on_dt,
        div,
        alm_line,
        ia_line,
        &alm_concat,
        &gls.prod_concat,
        prin_amt,
        int_amt,
        ost_bal,
        ftp_dates.nxt_rep_dt,
        ftp_dates.lst_rep_dt,
        log,
    );

    if alm_line == "NONE" {
        op_lines
            .concat_lines
            .push(get_concat_line(acc, &fv_gl, &alm_concat));
    }
    op_lines
}

fn get_concat_line(acc: &InputAccount, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("SecurityComposition|");
    op_line.push_str(&acc.deal_no);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}

fn get_line(
    acc: &InputAccount,
    as_on_dt: NaiveDate,
    div: &str,
    alm_line: &str,
    ia_line: &str,
    concat: &str,
    prod_concat: &str,
    prin_amt: f64,
    int_amt: f64,
    ost_bal: f64,
    ftp_nxt_rep_dt: NaiveDate,
    ftp_lst_rep_dt: NaiveDate,
    log: &Logger,
) -> String {
    let mut op_line = String::new();
    let nxt_rep_dt = NaiveDate::parse_from_str(&acc.nxt_rep_dt, "%d-%m-%Y").unwrap();
    op_line.push_str(&acc.deal_no);
    op_line.push('|');
    op_line.push_str(&acc.short_name.trim());
    op_line.push('|');
    if as_on_dt > nxt_rep_dt {
        op_line.push_str(&"31-12-2999".to_string());
    } else {
        op_line.push_str(&acc.nxt_rep_dt);
    }
    op_line.push('|');
    op_line.push_str(&acc.call_dt);
    op_line.push('|');
    op_line.push_str(&acc.put_dt);
    op_line.push('|');
    op_line.push_str(&acc.deal_dt);
    op_line.push('|');
    op_line.push_str(&acc.portfolio);
    op_line.push('|');
    op_line.push_str(&acc.deal_rt);
    op_line.push('|');
    op_line.push_str(&acc.org_fv);
    op_line.push('|');
    op_line.push_str(&acc.os_fv);
    op_line.push('|');
    op_line.push_str(&acc.org_cv);
    op_line.push('|');
    op_line.push_str(&acc.accr_int);
    op_line.push('|');
    op_line.push_str(&acc.book_yield);
    op_line.push('|');

    let int_basis: i32 = match acc.intr_prac.to_uppercase().as_str() {
        "30EBY360" => 5,
        "ACTUALBY365" => 4,
        "ANNUAL" => 4,
        "ANNUALBY2" => 5,
        _ => 0,
    };
    op_line.push_str(&int_basis.to_string());
    op_line.push('|');

    op_line.push_str(&acc.avg_os_vd);
    op_line.push('|');
    op_line.push_str(&acc.avg_os_vd);
    op_line.push('|');
    op_line.push_str(&prin_amt.to_string());
    op_line.push('|');
    op_line.push_str(&acc.org_cv);
    op_line.push('|');
    op_line.push_str(&acc.coup);
    op_line.push('|');
    op_line.push_str(&acc.nxt_intr_dt);
    op_line.push('|');
    op_line.push_str(&acc.gl_cd);
    op_line.push('|');
    op_line.push_str(&acc.vw_val_dt);
    op_line.push('|');
    op_line.push_str(&acc.short_name.trim());
    op_line.push('|');
    op_line.push_str(&acc.prod_desc);
    op_line.push('|');
    op_line.push_str(&acc.prod);
    op_line.push('|');
    op_line.push_str(&acc.lst_intr_dt);
    op_line.push_str("|NULL|");
    op_line.push_str(&acc.intr_app_freq);
    op_line.push('|');
    op_line.push_str(&acc.master_val_dt);
    op_line.push_str("|DAILY|||");
    op_line.push_str(&acc.currency);
    op_line.push('|');

    let org_term: f64 = if let (Ok(st_dt), Ok(end_dt)) = (
        NaiveDate::parse_from_str(&acc.master_val_dt, "%d-%m-%Y"),
        NaiveDate::parse_from_str(&acc.mat_dt, "%d-%m-%Y"),
    ) {
        if st_dt < end_dt {
            ((num_days_start_to_end(st_dt, end_dt) as f64) / 365.0) * 12.0
        } else {
            log_error!(
                log,
                "`start date`: `{}` is greater than `end date`: `{}` for account: `{}`",
                st_dt,
                end_dt,
                acc.deal_no
            );
            0.0
        }
    } else {
        0.0
    };
    op_line.push_str(&org_term.to_string());
    op_line.push('|');

    let acc_basis: &str = match int_basis {
        1 => "B",
        2 => "D",
        3 => "E",
        4 => "F",
        5 => "B",
        6 => "E",
        7 => "F",
        0 => "F",
        9 => "2",
        _ => "F",
    };
    op_line.push_str(acc_basis);
    op_line.push('|');

    op_line.push_str(prod_concat);
    op_line.push('|');
    op_line.push_str(concat);
    op_line.push('|');
    op_line.push_str(div);
    op_line.push('|');
    op_line.push_str(alm_line);
    op_line.push('|');
    op_line.push_str(ia_line);
    op_line.push('|');

    let invst_typ = &acc.prod_desc;
    let comp_freq: i32 = match invst_typ.as_str() {
        "COMMP" => 3000,
        "TBILL" => 3000,
        "ZCORP" => 3000,
        "ZECBD" => 3000,
        _ => 0,
    };
    op_line.push_str(&comp_freq.to_string());
    op_line.push('|');

    let nxt_comp_dt: Option<NaiveDate> = match invst_typ.as_str() {
        "COMMP" => as_on_dt.succ_opt(),
        "TBILL" => as_on_dt.succ_opt(),
        "ZCORP" => as_on_dt.succ_opt(),
        "ZECBD" => as_on_dt.succ_opt(),
        _ => None,
    };
    let nxt_comp_dt = match nxt_comp_dt {
        Some(dt) => dt.format("%d-%m-%Y").to_string(),
        None => "NULL".to_string(),
    };
    op_line.push_str(&nxt_comp_dt);
    op_line.push('|');

    let rt_chng_freq: i32 = match invst_typ.as_str() {
        "ZCORP" => 3000,
        "GSEC-FRB" => 6,
        _ => 0,
    };
    op_line.push_str(&rt_chng_freq.to_string());
    op_line.push('|');

    let int_type = &acc.intr_typ;
    let put_date = NaiveDate::parse_from_str(&acc.put_dt, "%d-%m-%Y");
    let nxt_rep_dt = NaiveDate::parse_from_str(&acc.nxt_rep_dt, "%d-%m-%Y");
    let rt_flag: &str = if (int_type == "Fixed" || int_type == "None") && put_date.is_err() {
        "F"
    } else if (int_type == "Fixed" || int_type == "None") && put_date.is_ok() {
        "P"
    } else if int_type == "Floating" && nxt_rep_dt.is_err() {
        "V"
    } else if int_type == "Floating" && nxt_rep_dt.is_ok() {
        "A"
    } else {
        "NULL"
    };
    op_line.push_str(rt_flag);
    op_line.push('|');
    op_line.push_str(&invst_typ);
    op_line.push('|');

    let mat_dt = NaiveDate::parse_from_str(&acc.mat_dt, "%d-%m-%Y");
    let n_c_dt = NaiveDate::parse_from_str(&acc.nxt_intr_dt, "%d-%m-%Y");
    let nxt_pay_dt: Option<NaiveDate> = if invst_typ == "ZCORP" {
        if let Ok(dt) = n_c_dt {
            Some(dt)
        } else if mat_dt.is_ok()
            && mat_dt.expect("Error while parsing maturity date.")
                <= increment_date_by_months(as_on_dt, 1)
        {
            Some(mat_dt.expect("Unable to parse maturit date."))
        } else {
            Some(increment_date_by_months(as_on_dt, 1))
        }
    } else if invst_typ == "GSEC-FRB" {
        if let Ok(dt) = n_c_dt {
            Some(dt)
        } else {
            if let Ok(dt) = mat_dt {
                Some(dt)
            } else {
                None
            }
        }
    } else {
        None
    };

    if let Some(dt) = nxt_pay_dt {
        op_line.push_str(&dt.format("%d-%m-%Y").to_string());
    } else {
        op_line.push_str("NULL");
    }
    op_line.push('|');

    let prev_rep_dt: String = match invst_typ.as_str() {
        "ZCORP" => as_on_dt.format("%d-%m-%Y").to_string(),
        "GSEC-FRB" => acc.nxt_intr_dt.to_string(),
        _ => "NULL".to_string(),
    };
    op_line.push_str(&prev_rep_dt);
    op_line.push('|');

    let coup_freq = &acc.intr_app_freq;
    let int_pay_freq: &str = if invst_typ == "ZCORP" || coup_freq == "Monthly" {
        "1"
    } else if coup_freq == "Quarterly" {
        "3"
    } else if coup_freq == "HalfYearly" {
        "6"
    } else if coup_freq == "Yearly" {
        "12"
    } else {
        "0"
    };
    op_line.push_str(int_pay_freq);
    op_line.push('|');
    // Using deal_ytm as interest rate irrespective of prod_desc
    op_line.push_str(&acc.deal_ytm);
    op_line.push('|');
    op_line.push_str(&as_on_dt.format("%d-%m-%Y").to_string());
    op_line.push('|');
    op_line.push_str(&acc.portfolio_type);
    op_line.push('|');
    op_line.push_str(&acc.sec_grp);
    op_line.push('|');
    op_line.push_str(&acc.sec_type);
    op_line.push('|');
    op_line.push_str(&acc.sec_issuer);
    op_line.push('|');
    op_line.push_str(&acc.sec_guaranteed);
    op_line.push('|');
    op_line.push_str(&acc.mrkt);
    op_line.push('|');
    op_line.push_str(&acc.index_label);
    op_line.push('|');
    op_line.push_str(&acc.bd_categ);
    op_line.push('|');
    op_line.push_str(&acc.bd_type);
    op_line.push('|');
    op_line.push_str(&acc.listed);
    op_line.push('|');
    op_line.push_str(&acc.npa_class);
    op_line.push('|');
    op_line.push_str(&acc.entity);
    op_line.push('|');
    op_line.push_str(&acc.desk);
    op_line.push('|');
    op_line.push_str(&acc.acc_sec_igaap);
    op_line.push('|');
    op_line.push_str(&acc.os_cv_before_amort);
    op_line.push('|');
    op_line.push_str(&acc.os_cv_after_amort);
    op_line.push('|');
    op_line.push_str(&acc.mat_dt);
    op_line.push('|');
    op_line.push_str(&int_amt.to_string());
    op_line.push('|');
    op_line.push_str(&acc.flow_type);
    op_line.push('|');
    op_line.push_str(&acc.isin);
    op_line.push('|');
    op_line.push_str(&acc.wap_igaap);
    op_line.push('|');
    op_line.push_str(&ost_bal.to_string());
    op_line.push('|');
    op_line.push_str(&acc.contract_no);
    op_line.push('|');
    op_line.push_str(&acc.instr_id);
    op_line.push('|');
    op_line.push_str(&acc.parent_code);
    op_line.push('|');
    op_line.push_str(&acc.issuer_name);
    op_line.push('|');
    op_line.push_str(&acc.rating);
    op_line.push('|');
    op_line.push_str(&acc.tax_status);
    op_line.push('|');
    op_line.push_str(&acc.slr_nslr);
    op_line.push('|');
    op_line.push_str(&acc.deal_ytm);
    op_line.push('|');
    op_line.push_str(&acc.intr_app_freq);
    op_line.push('|');
    op_line.push_str(&acc.comp_freq);
    op_line.push('|');
    op_line.push_str(&acc.intr_prac);
    op_line.push('|');
    op_line.push_str(&acc.rt_spread);
    op_line.push('|');
    op_line.push_str(&acc.gl_cd);
    op_line.push('|');
    op_line.push_str(&acc.intr_typ);
    op_line.push('|');
    op_line.push_str(&acc.sec_issuance_date);
    op_line.push('|');
    op_line.push_str(&acc.coup);
    op_line.push('|');
    op_line.push_str(&acc.lst_intr_dt);
    op_line.push('|');
    op_line.push_str(&acc.nxt_intr_dt);
    op_line.push('|');
    op_line.push_str(&acc.amort_till_dt);
    op_line.push('|');
    op_line.push_str(&ftp_lst_rep_dt.format("%d-%m-%Y").to_string());
    op_line.push('|');
    op_line.push_str(&ftp_nxt_rep_dt.format("%d-%m-%Y").to_string());
    op_line.push('\n');
    op_line
}

fn get_alm_concat(ora_mis1: &str, ora_prod: &str, ora_gl: &str, ora_cat: &str) -> String {
    let mut alm_concat: String = String::new();
    alm_concat.push_str(ora_mis1);
    alm_concat.push('_');
    alm_concat.push_str(ora_prod);
    alm_concat.push('_');
    alm_concat.push_str(ora_gl);
    alm_concat.push('_');
    alm_concat.push_str(ora_cat);
    alm_concat
}
