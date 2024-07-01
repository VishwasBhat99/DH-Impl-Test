use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::calculate::*;
use cashflow_generator::cashflow_appenders::create_account_with_cashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use chrono::Duration;
use rbdate::date_from_timestamp;
use rbdate::{increment_date_by_months, num_days_start_to_end, NaiveDate};
use slog::Logger;
use std::collections::HashMap;

//Create a masterdate for common fields.
#[derive(Debug, Clone)]
pub struct MasterData {
    pub acc_open_date: NaiveDate,
    pub disbursement_amt: f64,
    pub intr_calc_freq: String,
    pub intr_rate: f64,
    pub outstanding_bal: f64,
    pub repricing_dt: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct FlowStruct {
    pub repayment_amt: f64,
    pub schdl_stdt: NaiveDate,
    pub num_of_flows: i64,
    pub intr_calc_basis: String,
    pub repay_freq: String,
    pub acc_open_date: NaiveDate,
}
#[derive(Debug, Clone)]
pub struct OutputCF {
    pub item_type: String,
    pub inst_dt: NaiveDate,
    pub inst_amt: f64,
    pub repricing_dt: NaiveDate,
    pub od_days: i64,
}
#[derive(Debug, Clone)]
pub struct PrinStruct {
    pub repay_amt: f64,
    pub shdl_st_dt: NaiveDate,
    pub num_of_flows: i64,
    pub int_calc_basis: String,
    pub repayment_freq: String,
}
#[derive(Debug, Clone)]
pub struct IntrStruct {
    pub repricing_dt: NaiveDate,
    pub num_of_flow: i64,
    pub shdl_st_dt: NaiveDate,
    pub int_calc_basis: String,
    pub repayment_freq: String,
}

pub fn process_ln2(
    input_rcrds: Vec<InputAccount>,
    as_on_date: NaiveDate,
    calc_ir_from_ason: String,
    _log: &Logger,
    _diag_log: &Logger,
    account_with_cashflows_prin: &mut Vec<AccountWithCashflows>,
    account_with_cashflows_od: &mut Vec<AccountWithCashflows>,
) {
    let mut cf: Vec<Vec<OutputCF>>;
    //Common data stored in master data.
    let master_data = MasterData {
        acc_open_date: input_rcrds[0]
            .acc_open_dt
            .unwrap_or(NaiveDate::from_ymd_opt(1970, 1, 1).expect("Could not get default date.")),
        disbursement_amt: input_rcrds[0].disbursement_amt,
        intr_calc_freq: input_rcrds[0].intr_comp_freq.to_owned(),
        intr_rate: input_rcrds[0].interest_rate.to_owned(),
        outstanding_bal: (input_rcrds[0].outstanding_amt).abs(),
        repricing_dt: input_rcrds[0]
            .repricing_dt
            .unwrap_or(NaiveDate::from_ymd(1970, 1, 1)),
    };
    if input_rcrds[0].emi_flag.to_uppercase() == "NONEI" {
        cf = generate_non_ei_cf(
            input_rcrds.to_owned(),
            master_data.to_owned(),
            as_on_date,
            calc_ir_from_ason,
        );
    } else {
        cf = generate_ei_cf(
            input_rcrds.to_owned(),
            master_data.to_owned(),
            as_on_date,
            calc_ir_from_ason,
        );
    }
    let mut cashflows_prin: Vec<Cashflow> = Vec::new();
    //Principal CF and Interest CF:
    let mut first_cf_prin: OutputCF = OutputCF {
        item_type: "NA".to_string(),
        inst_dt: NaiveDate::from_ymd(1970, 1, 1),
        inst_amt: 0.0,
        repricing_dt: NaiveDate::from_ymd(1970, 1, 1),
        od_days: 0,
    };
    for i in 0..cf[0].len() {
        if cf[0][i].repricing_dt > cf[0][i].inst_dt {
            cf[0][i].repricing_dt = cf[0][i].inst_dt;
        }
        //Check for the first account
        if i == 0 {
            first_cf_prin = cf[0][i].clone();
        }
        if cf[0][i].item_type == "P".to_string() {
            cashflows_prin.push(generate_cashflows(cf[0][i].inst_amt, 0.0, cf[0][i].inst_dt));
        } else {
            cashflows_prin.push(generate_cashflows(0.0, cf[0][i].inst_amt, cf[0][i].inst_dt));
        }
    }

    // OD CF
    let mut first_cf_od: OutputCF = OutputCF {
        item_type: "NA".to_string(),
        inst_dt: NaiveDate::from_ymd(1970, 1, 1),
        inst_amt: 0.0,
        repricing_dt: NaiveDate::from_ymd(1970, 1, 1),
        od_days: 0,
    };
    let mut cashflows_od: Vec<Cashflow> = Vec::new();
    for i in 0..cf[1].len() {
        if cf[1][i].od_days > 0 {
            if i == 0 {
                first_cf_od = cf[0][i].clone();
            }
            cashflows_od.push(generate_cashflows(cf[1][i].inst_amt, 0.0, cf[1][i].inst_dt));
        } else {
            cashflows_prin.push(generate_cashflows(cf[1][i].inst_amt, 0.0, cf[1][i].inst_dt));
        }
    }
    account_with_cashflows_prin.push(create_account_with_cashflows(
        &mut input_rcrds[0].to_owned(),
        first_cf_prin,
        cashflows_prin,
        true,
    ));
    account_with_cashflows_od.push(create_account_with_cashflows(
        &mut input_rcrds[0].to_owned(),
        first_cf_od,
        cashflows_od.to_owned(),
        false,
    ));
}

pub fn generate_ei_cf(
    input_rcrds: Vec<InputAccount>,
    master_data: MasterData,
    as_on_date: NaiveDate,
    cals_ir_from_ason: String,
) -> Vec<Vec<OutputCF>> {
    let cf: Vec<Vec<OutputCF>>;
    let mut flow_struct: Vec<FlowStruct> = Vec::new();
    for i in 0..input_rcrds.len() {
        flow_struct.push(FlowStruct {
            repayment_amt: input_rcrds[i].flow_amt,
            schdl_stdt: input_rcrds[i]
                .princ_sch_srt_dt
                .unwrap_or(NaiveDate::from_ymd(1970, 1, 1)),
            num_of_flows: input_rcrds[i].princ_flow_num,
            intr_calc_basis: input_rcrds[i].princ_pay_freq.to_owned(),
            repay_freq: input_rcrds[i].princ_pay_freq.to_owned(),
            acc_open_date: input_rcrds[i]
                .acc_open_dt
                .unwrap_or(NaiveDate::from_ymd(1970, 1, 1)),
        });
    }
    if cals_ir_from_ason == "Y".to_string() {
        cf = generate_emi_cf_from_as_on(master_data, flow_struct, as_on_date);
    } else {
        cf = generate_emi_cf(master_data, flow_struct, as_on_date);
    }
    cf
}

pub fn generate_emi_cf_from_as_on(
    mstr_data: MasterData,
    flow_struct: Vec<FlowStruct>,
    ason: NaiveDate,
) -> Vec<Vec<OutputCF>> {
    let mut processing_over: bool = false;
    let mut lst_cf: Vec<Vec<OutputCF>> = Vec::new();
    let mut lst_prin_cf: Vec<OutputCF> = Vec::new();
    let mut lst_od: Vec<OutputCF> = Vec::new();
    let mut over_due_diff: f64;
    let mut cur_outstanding = (mstr_data.outstanding_bal * 100.0).round() / 100.0;
    let mut pend_prin_amt = (mstr_data.disbursement_amt * 100.0).round() / 100.0;
    let mut intr_inst_amt: f64;
    let mut prin_inst_amt: f64;
    let mut prvs_inst_date: NaiveDate = NaiveDate::from_ymd(1970, 1, 1);
    let mut is_first_cf = true;
    let mut cur_inst_date: NaiveDate;
    let mut flow_count: i64;
    if pend_prin_amt < mstr_data.outstanding_bal {
        pend_prin_amt = (mstr_data.outstanding_bal * 100.0).round() / 100.0;
    }
    for i in 0..flow_struct.len() {
        flow_count = flow_struct[i].num_of_flows;
        cur_inst_date = flow_struct[i].schdl_stdt;
        prvs_inst_date = flow_struct[i].acc_open_date;
        while flow_count > 0 && processing_over == false {
            intr_inst_amt = (calculate_interest(
                pend_prin_amt,
                mstr_data.intr_rate,
                num_days_start_to_end(cur_inst_date, prvs_inst_date),
                flow_struct[i].intr_calc_basis.to_owned(),
                cur_inst_date,
                flow_struct[i].repay_freq.to_owned(),
                false,
            ) * 100.0)
                .round()
                / 100.0;
            prin_inst_amt =
                ((flow_struct[i].repayment_amt - intr_inst_amt) * 100.0).round() / 100.0;

            if prin_inst_amt > cur_outstanding {
                prin_inst_amt = cur_outstanding;
            }
            if prin_inst_amt > 0.0 {
                pend_prin_amt = ((pend_prin_amt - prin_inst_amt) * 100.0).round() / 100.0
            }
            if cur_inst_date > ason {
                intr_inst_amt = (calculate_interest(
                    cur_outstanding,
                    mstr_data.intr_rate,
                    num_days_start_to_end(cur_inst_date, prvs_inst_date),
                    flow_struct[i].intr_calc_basis.to_owned(),
                    cur_inst_date,
                    flow_struct[i].repay_freq.to_owned(),
                    false,
                ) * 100.0)
                    .round()
                    / 100.0;
                prin_inst_amt =
                    ((flow_struct[i].repayment_amt - intr_inst_amt) * 100.0).round() / 100.0;

                if is_first_cf {
                    intr_inst_amt = (calculate_interest(
                        cur_outstanding,
                        mstr_data.intr_rate,
                        num_days_start_to_end(cur_inst_date, ason),
                        flow_struct[i].intr_calc_basis.to_owned(),
                        cur_inst_date,
                        flow_struct[i].repay_freq.to_owned(),
                        true,
                    ) * 100.0)
                        .round()
                        / 100.0;
                }
                if prin_inst_amt > cur_outstanding {
                    prin_inst_amt = cur_outstanding;
                    processing_over = true;
                }
                if prin_inst_amt > 0.0 {
                    lst_prin_cf.push(OutputCF {
                        item_type: "P".to_string(),
                        inst_dt: cur_inst_date,
                        inst_amt: prin_inst_amt,
                        repricing_dt: mstr_data.repricing_dt,
                        od_days: 0,
                    });
                    lst_prin_cf.push(OutputCF {
                        item_type: "I".to_string(),
                        inst_dt: cur_inst_date,
                        inst_amt: intr_inst_amt,
                        repricing_dt: mstr_data.repricing_dt,
                        od_days: 0,
                    });
                    cur_outstanding = ((cur_outstanding - prin_inst_amt) * 100.0).round() / 100.0;
                }
                is_first_cf = false;
            } else {
                over_due_diff = ((cur_outstanding - pend_prin_amt) * 100.0).round() / 100.0;
                if over_due_diff > 0.0 {
                    cur_outstanding = ((cur_outstanding - over_due_diff) * 100.0).round() / 100.0;
                    lst_od.push(OutputCF {
                        item_type: "OD".to_string(),
                        inst_dt: cur_inst_date,
                        inst_amt: over_due_diff,
                        repricing_dt: mstr_data.repricing_dt,
                        od_days: num_days_start_to_end(ason, cur_inst_date),
                    });
                }
            }
            prvs_inst_date = cur_inst_date;

            cur_inst_date = get_next_inst_date(cur_inst_date, flow_struct[i].repay_freq.to_owned());
            flow_count -= 1;
        }
    }

    if cur_outstanding > 0.0 {
        lst_prin_cf.push(OutputCF {
            item_type: "P".to_string(),
            inst_dt: prvs_inst_date,
            inst_amt: cur_outstanding,
            repricing_dt: mstr_data.repricing_dt,
            od_days: 0,
        });
    }
    lst_cf.push(lst_prin_cf);
    lst_cf.push(lst_od);

    lst_cf
}

pub fn generate_emi_cf(
    mstr_data: MasterData,
    flow_struct: Vec<FlowStruct>,
    ason: NaiveDate,
) -> Vec<Vec<OutputCF>> {
    let mut processing_over = false;
    let mut lst_cf: Vec<Vec<OutputCF>> = Vec::new();
    let mut lst_prin_cf: Vec<OutputCF> = Vec::new();
    let mut lst_od: Vec<OutputCF> = Vec::new();
    let mut over_due_diff: f64;
    let mut cur_outstanding = (mstr_data.outstanding_bal * 100.0).round() / 100.0;
    let mut pend_prin_amt = (mstr_data.disbursement_amt * 100.0).round() / 100.0;
    let mut intr_inst_amt: f64;
    let mut prin_inst_amt: f64;
    let mut intr_calc_dur;
    let mut prvs_inst_date: NaiveDate = NaiveDate::from_ymd(1970, 1, 1);
    let mut cur_inst_date: NaiveDate;
    let mut flow_count: i64;
    if pend_prin_amt < mstr_data.outstanding_bal {
        pend_prin_amt = (mstr_data.outstanding_bal * 100.0).round() / 100.0;
    }
    for i in 0..flow_struct.len() {
        flow_count = flow_struct[i].num_of_flows;
        cur_inst_date = flow_struct[i].schdl_stdt;
        while flow_count > 0 && processing_over == false {
            intr_calc_dur = get_month_equivalent(
                flow_struct[i].repay_freq.to_owned(),
                flow_struct[i].intr_calc_basis.to_owned(),
                cur_inst_date,
            );
            intr_inst_amt =
                (calculate_si_by_months(pend_prin_amt, mstr_data.intr_rate, intr_calc_dur) * 100.0)
                    .round()
                    / 100.0;
            prin_inst_amt =
                ((flow_struct[i].repayment_amt - intr_inst_amt) * 100.0).round() / 100.0;
            if prin_inst_amt > cur_outstanding {
                prin_inst_amt = cur_outstanding;
            }
            if prin_inst_amt > 0.0 {
                pend_prin_amt = ((pend_prin_amt - prin_inst_amt) * 100.0).round() / 100.0
            }
            if cur_inst_date > ason {
                intr_inst_amt =
                    (calculate_si_by_months(cur_outstanding, mstr_data.intr_rate, intr_calc_dur)
                        * 100.0)
                        .round()
                        / 100.0;
                prin_inst_amt =
                    ((flow_struct[i].repayment_amt - intr_inst_amt) * 100.0).round() / 100.0;

                if prin_inst_amt > cur_outstanding {
                    prin_inst_amt = cur_outstanding;
                    processing_over = true;
                }
                if prin_inst_amt > 0.0 {
                    lst_prin_cf.push(OutputCF {
                        item_type: "P".to_string(),
                        inst_dt: cur_inst_date,
                        inst_amt: prin_inst_amt,
                        repricing_dt: mstr_data.repricing_dt,
                        od_days: 0,
                    });
                    lst_prin_cf.push(OutputCF {
                        item_type: "I".to_string(),
                        inst_dt: cur_inst_date,
                        inst_amt: intr_inst_amt,
                        repricing_dt: mstr_data.repricing_dt,
                        od_days: 0,
                    });
                    cur_outstanding = ((cur_outstanding - prin_inst_amt) * 100.0).round() / 100.0;
                }
            } else {
                over_due_diff = ((cur_outstanding - pend_prin_amt) * 100.0).round() / 100.0;
                if over_due_diff > 0.0 {
                    cur_outstanding = ((cur_outstanding - over_due_diff) * 100.0).round() / 100.0;
                    lst_od.push(OutputCF {
                        item_type: "OD".to_string(),
                        inst_dt: cur_inst_date,
                        inst_amt: over_due_diff,
                        repricing_dt: mstr_data.repricing_dt,
                        od_days: num_days_start_to_end(ason, cur_inst_date),
                    });
                }
            }
            prvs_inst_date = cur_inst_date;
            cur_inst_date = get_next_inst_date(cur_inst_date, flow_struct[i].repay_freq.to_owned());
            flow_count -= 1;
        }
    }

    if cur_outstanding > 0.0 {
        lst_prin_cf.push(OutputCF {
            item_type: "P".to_string(),
            inst_dt: prvs_inst_date,
            inst_amt: cur_outstanding,
            repricing_dt: mstr_data.repricing_dt,
            od_days: 0,
        });
    }

    lst_cf.push(lst_prin_cf);
    lst_cf.push(lst_od);
    lst_cf
}

pub fn generate_non_ei_cf(
    input_rcrds: Vec<InputAccount>,
    master_data: MasterData,
    as_on_date: NaiveDate,
    calc_ir_from_ason: String,
) -> Vec<Vec<OutputCF>> {
    let mut prin_struct: Vec<PrinStruct> = Vec::new();
    let mut intr_struct: Vec<IntrStruct> = Vec::new();
    let cf: Vec<Vec<OutputCF>>;
    for i in 0..input_rcrds.len() {
        prin_struct.push(PrinStruct {
            repay_amt: input_rcrds[i].flow_amt.abs(),
            shdl_st_dt: input_rcrds[i]
                .princ_sch_srt_dt
                .unwrap_or(NaiveDate::from_ymd(1970, 1, 1)),
            num_of_flows: input_rcrds[i].princ_flow_num,
            int_calc_basis: input_rcrds[i].int_calc_basis.to_owned(),
            repayment_freq: input_rcrds[i].princ_pay_freq.to_owned(),
        });
        intr_struct.push(IntrStruct {
            repricing_dt: input_rcrds[i].repricing_dt.unwrap_or(
                NaiveDate::from_ymd_opt(1970, 1, 1).expect("Could not get default date."),
            ),
            num_of_flow: input_rcrds[i].intr_flow_num,
            shdl_st_dt: input_rcrds[i].intr_sch_srt_dt.unwrap_or(
                NaiveDate::from_ymd_opt(1970, 1, 1).expect("Could not get default date."),
            ),
            int_calc_basis: input_rcrds[i].int_calc_basis.to_owned(),
            repayment_freq: input_rcrds[i].intr_pay_freq.to_owned(),
        });
    }

    if calc_ir_from_ason == "Y".to_string() {
        cf = generate_non_emi_cf_from_as_on(master_data, prin_struct, intr_struct, as_on_date);
    } else {
        cf = generate_non_emi_cf(master_data, prin_struct, intr_struct, as_on_date)
    }
    cf
}

pub fn generate_non_emi_cf_from_as_on(
    master_data: MasterData,
    prin_struct: Vec<PrinStruct>,
    intr_struct: Vec<IntrStruct>,
    as_on_date: NaiveDate,
) -> Vec<Vec<OutputCF>> {
    //GenerateNonEMICFFromAsOn
    let mut processing_over = false;
    let mut lst_cf: Vec<Vec<OutputCF>> = Vec::new();
    let mut lst_prin_cf: Vec<OutputCF> = Vec::new();
    let mut lst_od: Vec<OutputCF> = Vec::new();

    let mut over_due_diff;
    let mut cur_outstanding = (master_data.outstanding_bal * 100.0).round() / 100.0;
    let mut pend_prin_amt = (master_data.disbursement_amt * 100.0).round() / 100.0;
    let mut prin_inst_amt;
    let mut intr_inst_amt;
    let mut prev_inst_date;
    let mut cur_inst_date: NaiveDate;
    let mut is_first_cf = true;
    let mut flow_count;

    if pend_prin_amt < master_data.outstanding_bal {
        pend_prin_amt = (master_data.outstanding_bal * 100.0).round() / 100.0;
    }

    for i in 0..prin_struct.len() {
        flow_count = prin_struct[i].num_of_flows;
        cur_inst_date = prin_struct[i].shdl_st_dt;
        while flow_count > 0 && !processing_over {
            prin_inst_amt = (prin_struct[i].repay_amt * 100.0).round() / 100.0;
            if prin_inst_amt > cur_outstanding {
                prin_inst_amt = cur_outstanding;
            }
            if prin_inst_amt > 0.0 {
                pend_prin_amt = ((pend_prin_amt - prin_inst_amt) * 100.0).round() / 100.0;
            }
            if cur_inst_date > as_on_date {
                prin_inst_amt = ((prin_struct[i].repay_amt) * 100.0).round() / 100.0;
                if prin_inst_amt > cur_outstanding {
                    prin_inst_amt = cur_outstanding;
                    processing_over = true;
                }
                if prin_inst_amt > 0.0 {
                    //Add to principal cashflow.
                    lst_prin_cf.push(OutputCF {
                        item_type: "P".to_string(),
                        inst_dt: cur_inst_date,
                        inst_amt: prin_inst_amt,
                        repricing_dt: master_data.repricing_dt,
                        od_days: 0,
                    });

                    cur_outstanding -= prin_inst_amt;
                }
            } else {
                //Calculate overdue.
                over_due_diff = ((cur_outstanding - pend_prin_amt) * 100.0).round() / 100.0;
                if over_due_diff > 0.0 {
                    cur_outstanding -= over_due_diff;
                    lst_od.push(OutputCF {
                        item_type: "OD".to_string(),
                        inst_dt: cur_inst_date,
                        inst_amt: over_due_diff,
                        repricing_dt: master_data.repricing_dt,
                        od_days: num_days_start_to_end(as_on_date, cur_inst_date),
                    });
                }
            }
            prev_inst_date = cur_inst_date;
            cur_inst_date =
                get_next_inst_date(cur_inst_date, prin_struct[i].repayment_freq.to_owned());
            flow_count -= 1;
        }
    }
    if cur_outstanding > 0.0 {
        lst_prin_cf.push(OutputCF {
            item_type: "P".to_string(),
            inst_dt: as_on_date,
            inst_amt: cur_outstanding,
            repricing_dt: master_data.repricing_dt,
            od_days: 0,
        });
    }

    let mut count = 0;
    prev_inst_date = as_on_date;
    cur_outstanding = master_data.outstanding_bal;
    for i in 0..intr_struct.len() {
        flow_count = intr_struct[i].num_of_flow;
        cur_inst_date = intr_struct[i].shdl_st_dt;
        while flow_count > 0 && count < lst_prin_cf.len() {
            if cur_inst_date > as_on_date {
                let mut cal_int_flag = true;
                if is_first_cf {
                    intr_inst_amt = calculate_interest(
                        cur_outstanding,
                        master_data.intr_rate,
                        num_days_start_to_end(cur_inst_date, as_on_date),
                        intr_struct[i].int_calc_basis.to_owned(),
                        cur_inst_date,
                        intr_struct[i].repayment_freq.to_owned(),
                        cal_int_flag,
                    );
                } else {
                    cal_int_flag = false;
                    intr_inst_amt = calculate_interest(
                        cur_outstanding,
                        master_data.intr_rate,
                        num_days_start_to_end(cur_inst_date, prev_inst_date),
                        intr_struct[i].int_calc_basis.to_owned(),
                        cur_inst_date,
                        intr_struct[i].repayment_freq.to_owned(),
                        cal_int_flag,
                    );
                }
                //Add to interest cf.
                lst_prin_cf.push(OutputCF {
                    item_type: "I".to_string(),
                    inst_dt: cur_inst_date,
                    inst_amt: intr_inst_amt,
                    repricing_dt: master_data.repricing_dt,
                    od_days: 0,
                });

                if cur_inst_date >= lst_prin_cf[lst_prin_cf.len() - 1].inst_dt {
                    cur_outstanding = cur_outstanding - lst_prin_cf[lst_prin_cf.len() - 1].inst_amt;
                    count += 1;
                }
                is_first_cf = false;
                flow_count -= 1;
            }
            prev_inst_date = cur_inst_date;
            cur_inst_date =
                get_next_inst_date(cur_inst_date, intr_struct[i].repayment_freq.to_owned());
        }
    }
    lst_cf.push(lst_prin_cf);
    lst_cf.push(lst_od);
    return lst_cf;
}

pub fn generate_non_emi_cf(
    mstr_data: MasterData,
    prin_struct: Vec<PrinStruct>,
    intr_struct: Vec<IntrStruct>,
    as_on_date: NaiveDate,
) -> Vec<Vec<OutputCF>> {
    //GenerateNonEMICFFromAsOn
    let mut processing_over = false;
    let mut lst_cf: Vec<Vec<OutputCF>> = Vec::new();
    let mut lst_prin_cf: Vec<OutputCF> = Vec::new();
    let mut lst_od: Vec<OutputCF> = Vec::new();

    let mut over_due_diff;
    let mut cur_outstanding = (mstr_data.outstanding_bal * 100.0).round() / 100.0;
    let mut pend_prin_amt = (mstr_data.disbursement_amt * 100.0).round() / 100.0;
    let mut prin_inst_amt;
    let mut intr_inst_amt;
    let mut intr_calc_dur;
    let mut prev_inst_date;
    let mut cur_inst_date: NaiveDate;
    let mut flow_count;

    if pend_prin_amt < mstr_data.outstanding_bal {
        pend_prin_amt = (mstr_data.outstanding_bal * 100.0).round() / 100.0;
    }

    for i in 0..prin_struct.len() {
        flow_count = prin_struct[i].num_of_flows;
        cur_inst_date = prin_struct[i].shdl_st_dt;
        while flow_count > 0 && !processing_over {
            prin_inst_amt = (prin_struct[i].repay_amt * 100.0).round() / 100.0;
            if prin_inst_amt > cur_outstanding {
                prin_inst_amt = cur_outstanding;
            }
            if prin_inst_amt > 0.0 {
                pend_prin_amt = ((pend_prin_amt - prin_inst_amt) * 100.0).round() / 100.0;
            }
            if cur_inst_date > as_on_date {
                prin_inst_amt = ((prin_struct[i].repay_amt) * 100.0).round() / 100.0;
                if prin_inst_amt > cur_outstanding {
                    prin_inst_amt = cur_outstanding;
                    processing_over = true;
                }
                if prin_inst_amt > 0.0 {
                    //Add to principal cashflow.
                    lst_prin_cf.push(OutputCF {
                        item_type: "P".to_string(),
                        inst_dt: cur_inst_date,
                        inst_amt: prin_inst_amt,
                        repricing_dt: mstr_data.repricing_dt,
                        od_days: 0,
                    });

                    cur_outstanding -= prin_inst_amt;
                }
            } else {
                //Calculate overdue.
                over_due_diff = ((cur_outstanding - pend_prin_amt) * 100.0).round() / 100.0;
                if over_due_diff > 0.0 {
                    cur_outstanding -= over_due_diff;
                    lst_od.push(OutputCF {
                        item_type: "OD".to_string(),
                        inst_dt: cur_inst_date,
                        inst_amt: over_due_diff,
                        repricing_dt: mstr_data.repricing_dt,
                        od_days: num_days_start_to_end(as_on_date, cur_inst_date),
                    });
                }
            }
            prev_inst_date = cur_inst_date;
            cur_inst_date =
                get_next_inst_date(cur_inst_date, prin_struct[i].repayment_freq.to_owned());
            flow_count -= 1;
        }
    }
    if cur_outstanding > 0.0 {
        lst_prin_cf.push(OutputCF {
            item_type: "P".to_string(),
            inst_dt: as_on_date,
            inst_amt: cur_outstanding,
            repricing_dt: mstr_data.repricing_dt,
            od_days: 0,
        });
    }

    let mut count = 0;
    prev_inst_date = mstr_data.acc_open_date;
    cur_outstanding = mstr_data.outstanding_bal;
    for i in 0..intr_struct.len() {
        flow_count = intr_struct[i].num_of_flow;
        cur_inst_date = intr_struct[i].shdl_st_dt;
        while flow_count > 0 && count < lst_prin_cf.len() {
            intr_calc_dur = get_month_equivalent(
                intr_struct[i].repayment_freq.to_owned(),
                prin_struct[i].int_calc_basis.to_owned(),
                cur_inst_date,
            );

            if cur_inst_date > as_on_date {
                intr_inst_amt =
                    calculate_si_by_months(cur_outstanding, mstr_data.intr_rate, intr_calc_dur);
                lst_prin_cf.push(OutputCF {
                    item_type: "I".to_string(),
                    inst_dt: cur_inst_date,
                    inst_amt: intr_inst_amt,
                    repricing_dt: mstr_data.repricing_dt,
                    od_days: 0,
                });

                if cur_inst_date >= lst_prin_cf[lst_prin_cf.len() - 1].inst_dt {
                    cur_outstanding = cur_outstanding - lst_prin_cf[lst_prin_cf.len() - 1].inst_amt;
                    count += 1;
                }
                flow_count -= 1;
            }
            prev_inst_date = cur_inst_date;
            cur_inst_date =
                get_next_inst_date(cur_inst_date, intr_struct[i].repayment_freq.to_owned());
        }
    }
    lst_cf.push(lst_prin_cf);
    lst_cf.push(lst_od);
    return lst_cf;
}

pub fn get_next_inst_date(prev_inst_date: NaiveDate, freq: String) -> NaiveDate {
    let nxt_inst_dt = match freq.as_str() {
        "D" => prev_inst_date + (Duration::days(1)),
        "W" => prev_inst_date + (Duration::days(7)),
        "F" => prev_inst_date + (Duration::days(14)),
        "M" | "1" => increment_date_by_months(prev_inst_date, 1),
        "Q" | "2" => increment_date_by_months(prev_inst_date, 3),
        "H" | "3" => increment_date_by_months(prev_inst_date, 6),
        "Y" | "4" => increment_date_by_months(prev_inst_date, 12),
        _ => increment_date_by_months(prev_inst_date, 1),
    };
    nxt_inst_dt
}
