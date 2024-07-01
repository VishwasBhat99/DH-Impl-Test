extern crate serde;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillsData {
    pub acc_no: String,
    pub bill_id: String,
    pub branch_cd: String,
    pub cust_no: String,
    pub ucc_id: String,
    pub ccy: String,
    pub gl_cd: String,
    pub prod_cd: String,
    pub acc_open_dt: String,
    pub curr_out_bal: String,
    pub curr_out_bal_lcy: String,
    pub original_bill_amt: String,
    pub mat_amt: String,
    pub int_rt: String,
    pub mat_dt: String,
    pub nego_strt_dt: String,
    pub int_accrued: String,
    pub int_realised: String,
    pub ext_rt_agency_id: String,
    pub cust_rt_ext: String,
    pub cust_rt_int: String,
    pub npa_class: String,
    pub prov_amt: String,
    pub prov_dt: String,
    pub cust_const_cd: String,
    pub a1: String,
    pub a2: String,
    pub a3: String,
    pub a4: String,
    pub a5: String,
    pub a6: String,
    pub a7: String,
    pub a8: String,
    pub a9: String,
    pub a10: String,
}

#[derive(Debug, Clone, Default)]
pub struct BillsReqData {
    pub bill_id: String,
    pub acc_no: String,
    pub curr_out_bal_lcy: String,
    pub mat_dt: String,
    pub int_accrued: String,
    pub bill_ccy: String,
    pub nego_strt_dt: String,
}

impl BillsReqData {
    pub fn new(bills_data: &BillsData, as_on_date: NaiveDate) -> BillsReqData {
        BillsReqData {
            bill_id: bills_data.bill_id.to_string(),
            acc_no: bills_data.acc_no.to_string(),
            curr_out_bal_lcy: bills_data.curr_out_bal_lcy.to_string(),
            mat_dt: bills_data.mat_dt.to_string(),
            int_accrued: bills_data.int_accrued.to_string(),
            bill_ccy: bills_data.ccy.to_string(),
            nego_strt_dt: NaiveDate::parse_from_str(bills_data.nego_strt_dt.as_str(), "%d-%b-%y")
                .unwrap_or(as_on_date)
                .format("%d-%m-%Y")
                .to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ODData {
    pub key_1: String,
    pub branch_no: String,
    pub curr_status: String,
    pub acct_type: String,
    pub int_cat: String,
    pub inv_type: String,
    pub currency: String,
    pub customer_no: String,
    pub cr_limit: String,
    pub curr_bal: String,
    pub int_available: String,
    pub acct_open_dt: String,
    pub int_from_dt: String,
    pub int_to_dt: String,
    pub no_dues: String,
    pub var_int_rate: String,
    pub rval_ind: String,
    pub lst_ovr_limit_date: String,
    pub cr_store_rate: String,
    pub dr_store_rate: String,
    pub gl_class_code: String,
    pub mop_type: String,
    pub instl_due_day: String,
    pub lending_status: String,
    pub npa_clsfn: String,
    pub name: String,
    pub cust_acc_no: String,
    pub prim_accnt: String,
    pub segment_code: String,
    pub industry_code: String,
    pub grup_code: String,
    pub bus_sector_code: String,
    pub tier_cust_type: String,
    pub a1: String,
    pub a2: String,
    pub a3: String,
    pub a4: String,
    pub a5: String,
    pub a6: String,
    pub a7: String,
    pub a8: String,
    pub a9: String,
    pub a10: String,
    pub gl_code: String,
    pub int_rate: String,
    pub curr_bal_lcy: String,
    pub as_on_date: String,
    pub account_status: String,
    pub a12: String,
    pub a13: String,
    pub a14: String,
    pub a15: String,
    pub a16: String,
}

pub fn format_output(od_data: &ODData) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        od_data.key_1.trim(),
        od_data.branch_no.trim(),
        od_data.curr_status.trim(),
        od_data.acct_type.trim(),
        od_data.int_cat.trim(),
        od_data.inv_type.trim(),
        od_data.currency.trim(),
        od_data.customer_no.trim(),
        od_data.cr_limit.trim(),
        od_data.curr_bal.trim(),
        od_data.int_available.trim(),
        od_data.acct_open_dt.trim(),
        od_data.int_from_dt.trim(),
        od_data.int_to_dt.trim(),
        od_data.no_dues.trim(),
        od_data.var_int_rate.trim(),
        od_data.rval_ind.trim(),
        od_data.lst_ovr_limit_date.trim(),
        od_data.cr_store_rate.trim(),
        od_data.dr_store_rate.trim(),
        od_data.gl_class_code.trim(),
        od_data.mop_type.trim(),
        od_data.instl_due_day.trim(),
        od_data.lending_status.trim(),
        od_data.npa_clsfn.trim(),
        od_data.name.trim(),
        od_data.cust_acc_no.trim(),
        od_data.prim_accnt.trim(),
        od_data.segment_code.trim(),
        od_data.industry_code.trim(),
        od_data.grup_code.trim(),
        od_data.bus_sector_code.trim(),
        od_data.tier_cust_type.trim(),
        od_data.a1.trim(),
        od_data.a2.trim(),
        od_data.a3.trim(),
        od_data.a4.trim(),
        od_data.a5.trim(),
        od_data.a6.trim(),
        od_data.a7.trim(),
        od_data.a8.trim(),
        od_data.a9.trim(),
        od_data.a10.trim(),
        od_data.gl_code.trim(),
        od_data.int_rate.trim(),
        od_data.curr_bal_lcy.trim(),
        od_data.as_on_date.trim(),
        od_data.account_status.trim(),
        od_data.a12.trim(),
        od_data.a13.trim(),
        od_data.a14.trim(),
        od_data.a15.trim(),
        od_data.a16.trim(),
    )
}